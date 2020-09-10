use anyhow::{bail, Result};
use chrono::NaiveDateTime;
use crossterm::{
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rusqlite::{params, Connection, OpenFlags};
use std::{
  env,
  io::{stdout, Write},
};
use tui::{
  backend::CrosstermBackend,
  layout::{Constraint, Direction, Layout},
  style::{Color, Style},
  widgets::{Block, Borders, Row, Table, TableState},
  Terminal,
};

#[derive(Debug)]
struct Log {
  id: i64,
  time: f64,
  host: String,
  pid: u32,
  level: u8,
  typ: u8,
  channel: String,
  process: String,
  message: String,
}

fn main() -> Result<()> {
  let path = match env::args().nth(1) {
    Some(p) => p,
    None => bail!("Usage: ellit [path to .lsw file]"),
  };
  let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;

  // setup terminal
  enable_raw_mode()?;
  let mut stdout = stdout();
  execute!(stdout, EnterAlternateScreen)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;
  terminal.hide_cursor()?;
  terminal.clear()?;

  loop {
    terminal.draw(|mut f| {
      let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(10), Constraint::Length(3)].as_ref())
        .split(f.size());

      let selected_style = Style::default().fg(Color::Green);
      let normal_style = Style::default().fg(Color::White);
      let header = ["   Time", "Pid", "Message"];
      let mut stmt = conn
        .prepare("SELECT rowid, time, host, pid, level, type, channel, process, message FROM log")
        .unwrap();
      let log_iter = stmt
        .query_map(params![], |row| {
          Ok(Log {
            id: row.get(0)?,
            time: row.get(1)?,
            host: row.get(2)?,
            pid: row.get(3)?,
            level: row.get(4)?,
            typ: row.get(5)?,
            channel: row.get(6)?,
            process: row.get(7)?,
            message: row.get(8)?,
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
      let rows = logs.iter().map(|r| Row::StyledData(r.iter(), normal_style));

      let t = Table::new(header.iter(), rows)
        .header_style(Style::default().fg(Color::Yellow))
        .header_gap(0)
        .column_spacing(1)
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[Constraint::Length(23), Constraint::Length(8), Constraint::Min(10)]);
      let mut ts = TableState::default();
      ts.select(Some(1));
      f.render_stateful_widget(t, chunks[0], &mut ts);

      let block = Block::default().title("- Message ").borders(Borders::TOP);
      f.render_widget(block, chunks[1]);

      let block = Block::default().title(" Nodes ").borders(Borders::ALL);
      f.render_widget(block, chunks[2]);
    })?;

    match block_wait_action() {
      Some(Action::Quit) => {
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        break;
      }
      _ => {}
    }
  }

  Ok(())
}

enum Action {
  Quit,
}

fn block_wait_action() -> Option<Action> {
  use crossterm::event::{read, Event, KeyCode, KeyEvent};

  loop {
    match read().unwrap() {
      Event::Key(KeyEvent {
        code: KeyCode::Char('q'),
        modifiers: _,
      }) => break Some(Action::Quit),
      Event::Resize(_, _) => break None,
      _ => {}
    }
  }
}
