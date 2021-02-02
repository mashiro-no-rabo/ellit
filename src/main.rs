use anyhow::{bail, Result};
use chrono::NaiveDateTime;
use crossterm::{
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rusqlite::{params, Connection, OpenFlags};
use std::{env, io::stdout};
use tui::{
  backend::CrosstermBackend,
  layout::{Alignment, Constraint, Direction, Layout},
  style::{Color, Style},
  text::{Span, Text},
  widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState, Tabs, Wrap},
  Terminal,
};

mod input;
use input::Action;

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
  fn levels(&self) -> String {
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

  let selected_style = Style::default().fg(Color::Green);
  let normal_style = Style::default().fg(Color::White);

  loop {
    let mut log_page_size = 0;

    terminal.draw(|f| {
      let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
          [
            Constraint::Min(5),
            Constraint::Length(app.message_height),
            Constraint::Length(3),
          ]
          .as_ref(),
        )
        .split(f.size());

      log_page_size = (chunks[0].height - 1) as u64;

      let mut stmt = conn
        .prepare(&format!(
          "SELECT time, pid, level, channel, message FROM log WHERE level IN ({}) LIMIT (?) OFFSET (?)",
          app.levels()
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
      let rows = logs
        .iter()
        .map(|log| Row::new(log.iter().map(|txt| Cell::from(txt.as_ref()).style(normal_style))));

      let t = Table::new(rows)
        .header(Row::new(vec!["   Time", "Pid", "Message"]).style(Style::default().fg(Color::Yellow)))
        .column_spacing(1)
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[Constraint::Length(23), Constraint::Length(8), Constraint::Min(10)]);

      let mut ts = TableState::default();
      ts.select(Some(app.selected));
      f.render_stateful_widget(t, chunks[0], &mut ts);

      let msg = match logs.get(app.selected) {
        Some(log) => log[2].as_ref(),
        None => "",
      };

      let msg_disp = Paragraph::new(Text::from(msg))
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::TOP))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
      f.render_widget(msg_disp, chunks[1]);

      let mut lvls = vec![];
      if app.info {
        lvls.push(Span::styled("[1] INFO", Style::default().bg(Color::Gray).fg(Color::White)).into())
      } else {
        lvls.push(Span::raw("[1] INFO").into())
      }
      if app.notice {
        lvls.push(Span::styled("[2] NOTICE", Style::default().bg(Color::Cyan).fg(Color::Black)).into())
      } else {
        lvls.push(Span::raw("[2] NOTICE").into())
      }
      if app.warning {
        lvls.push(Span::styled("[3] WARN", Style::default().bg(Color::Yellow).fg(Color::White)).into())
      } else {
        lvls.push(Span::raw("[3] WARN").into())
      }
      if app.error {
        lvls.push(Span::styled("[4] ERROR", Style::default().bg(Color::Red).fg(Color::White)).into())
      } else {
        lvls.push(Span::raw("[4] ERROR").into())
      }
      let tabs = Tabs::new(lvls).block(Block::default().title(" Ellit ").borders(Borders::ALL));
      f.render_widget(tabs, chunks[2]);

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
