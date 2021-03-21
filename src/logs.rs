use anyhow::Result;
use chrono::NaiveDateTime;
use rusqlite::{params, Connection, OpenFlags, NO_PARAMS};

// TODO: just take a "app state" reference

#[derive(Debug)]
pub struct Storage {
  conn: Connection,
  outdated: bool,
  page_size: u64,
  offset: u64,
  levels: String,
  logs: Vec<Log>,
  filtered_count: u32,
}

#[derive(Debug)]
struct Log {
  time: f64,
  pid: u32,
  level: u8,
  channel: String,
  message: String,
}

impl Storage {
  pub fn open(path: &str) -> Result<Self> {
    Ok(Self {
      conn: Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)?,
      outdated: true,
      page_size: 10, // will be updated on first query
      offset: 0,
      levels: "".to_string(),
      logs: vec![],
      filtered_count: 0,
    })
  }

  pub fn set_page_size(&mut self, ps: u64) {
    if self.page_size != ps {
      self.page_size = ps;
      self.outdated = true;
    }
  }

  pub fn set_levels_filter(&mut self, lvls: String) {
    if self.levels != lvls {
      self.levels = lvls;
      self.outdated = true;
    }
  }

  pub fn set_offset(&mut self, ofst: u64) {
    if self.offset != ofst {
      self.offset = ofst;
      self.outdated = true;
    }
  }

  fn query(&mut self) {
    let mut stmt = self
      .conn
      .prepare(&format!(
        "SELECT time, pid, level, channel, message FROM log WHERE level IN ({}) LIMIT (?) OFFSET (?)",
        self.levels
      ))
      .unwrap();

    self.logs = stmt
      .query_map(params![self.page_size.to_string(), self.offset.to_string()], |row| {
        Ok(Log {
          time: row.get(0)?,
          pid: row.get(1)?,
          level: row.get(2)?,
          channel: row.get(3)?,
          message: row.get(4)?,
        })
      })
      .unwrap()
      .map(|l| l.unwrap())
      .collect();

    self.filtered_count = self
      .conn
      .query_row(
        &format!("SELECT COUNT(*) FROM log WHERE level IN ({})", self.levels),
        NO_PARAMS,
        |r| r.get(0),
      )
      .unwrap();

    self.outdated = false;
  }

  // TODO: return referenced data
  pub fn logs_table(&mut self) -> Vec<[String; 3]> {
    if self.outdated {
      self.query();
    }

    self
      .logs
      .iter()
      .map(|log| {
        let dt = NaiveDateTime::from_timestamp(log.time.floor() as i64, 0);
        [
          dt.format("%Y-%m-%d %H:%M:%S").to_string(),
          log.pid.to_string(),
          log.message.clone(),
        ]
      })
      .collect()
  }

  pub fn logs_count(&mut self) -> u32 {
    if self.outdated {
      self.query();
    }

    self.filtered_count
  }
}
