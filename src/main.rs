use anyhow::{bail, Result};
use chrono::NaiveDateTime;
use crossterm::{
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rusqlite::{params, Connection, OpenFlags};
use std::{env, io::stdout};
use tui::{backend::CrosstermBackend, Terminal};

mod input;
use input::Action;

mod ui;

#[derive(Debug)]
struct App {
  offset: u64,
  count: usize,
  selected: usize,
  message_height: u16,
  // log levels
  info: bool,
  notice: bool,
  warning: bool,
  error: bool,
  // focus (enum)
}

impl Default for App {
  fn default() -> Self {
    Self {
      offset: 0,
      count: 0,
      selected: 0,
      message_height: 10,
      info: false,
      notice: false,
      warning: true,
      error: true,
    }
  }
}

impl App {
  fn levels_sql(&self) -> String {
    let mut lvls = vec![];
    if self.info {
      lvls.push("0");
    }
    if self.notice {
      lvls.push("1");
    }
    if self.warning {
      lvls.push("2");
    }
    if self.error {
      lvls.push("3");
    }
    lvls.join(",")
  }
}

#[derive(Debug)]
struct Log {
  time: f64,
  pid: u32,
  level: u8,
  channel: String,
  message: String,
}

fn main() -> Result<()> {
  let path = match env::args().nth(1) {
    Some(p) => p,
    None => bail!("Usage: ellit [path to .lsw file]"),
  };
  let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
  let mut app = App::default();

  // setup terminal
  enable_raw_mode()?;
  let mut stdout = stdout();
  execute!(stdout, EnterAlternateScreen)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;
  terminal.hide_cursor()?;
  terminal.clear()?;

  let mut ui = ui::Ui::new();

  loop {
    let mut log_page_size = 0;

    terminal.draw(|f| {
      ui.resize_main(app.message_height, f.size());

      log_page_size = ui.log_page_size() as u64;

      let mut stmt = conn
        .prepare(&format!(
          "SELECT time, pid, level, channel, message FROM log WHERE level IN ({}) LIMIT (?) OFFSET (?)",
          app.levels_sql()
        ))
        .unwrap();
      let log_iter = stmt
        .query_map(params![log_page_size.to_string(), app.offset.to_string()], |row| {
          Ok(Log {
            time: row.get(0)?,
            pid: row.get(1)?,
            level: row.get(2)?,
            channel: row.get(3)?,
            message: row.get(4)?,
          })
        })
        .unwrap();
      let logs: Vec<[String; 3]> = log_iter
        .map(|l| l.unwrap())
        .map(|log| {
          let dt = NaiveDateTime::from_timestamp(log.time.floor() as i64, 0);
          [
            dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            log.pid.to_string(),
            log.message,
          ]
        })
        .collect();

      ui.render_logs_table(f, &logs);
      ui.select_log(app.selected);

      let msg = match logs.get(app.selected) {
        Some(log) => log[2].as_ref(),
        None => "",
      };
      ui.render_log_message(f, msg);

      ui.render_level_filter(f, app.info, app.notice, app.warning, app.error);

      app.count = logs.len();
    })?;

    match input::block_wait_action() {
      Some(Action::Quit) => {
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        break;
      }
      Some(Action::NextLog) => {
        app.selected += 1;
        if app.selected >= app.count {
          app.selected = app.count - 1
        }
      }
      Some(Action::PrevLog) => match app.selected.checked_sub(1) {
        Some(x) => app.selected = x,
        None => app.selected = 0,
      },
      Some(Action::NextPageLogs) => app.offset += log_page_size,
      Some(Action::PrevPageLogs) => match app.offset.checked_sub(log_page_size) {
        Some(x) => app.offset = x,
        None => app.selected = 0,
      },
      Some(Action::TopLog) => {
        app.offset = 0;
      }
      Some(Action::BottomLog) => {
        //
      }
      Some(Action::ToggleInfo) => {
        app.info ^= true;
      }
      Some(Action::ToggleNotice) => {
        app.notice ^= true;
      }
      Some(Action::ToggleWarning) => {
        app.warning ^= true;
      }
      Some(Action::ToggleError) => {
        app.error ^= true;
      }
      Some(Action::IncMessageHeight) => {
        app.message_height += 3;
      }
      Some(Action::DecMessageHeight) => {
        app.message_height -= 3;
      }
      None => {
        // handle resize event
      }
    }
  }

  Ok(())
}
