use tui::{
  backend::Backend,
  layout::{Alignment, Constraint, Direction, Layout, Rect},
  style::{Color, Style},
  text::{Span, Text},
  widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState, Tabs, Wrap},
  Frame,
};

use crate::app::LogLevels;

#[derive(Debug)]
pub struct Ui {
  chunks: Vec<Rect>,
  pub message_height: u16,
  pub selected: usize,

  normal_style: Style,
  selected_style: Style,
}

impl Default for Ui {
  fn default() -> Self {
    Self {
      chunks: vec![],
      message_height: 10,
      selected: 0,
      normal_style: Style::default().fg(Color::White),
      selected_style: Style::default().fg(Color::Green),
    }
  }
}

impl Ui {
  pub fn resize_main(&mut self, area: Rect) {
    self.chunks = Layout::default()
      .direction(Direction::Vertical)
      .constraints(
        [
          Constraint::Min(5),
          Constraint::Length(self.message_height),
          Constraint::Length(3),
        ]
        .as_ref(),
      )
      .split(area);
  }

  pub fn log_page_size(&self) -> u16 {
    // title uses 1 line, border uses 2 lines vertically
    self.chunks[0].height - 3
  }

  pub fn render_logs_table<B: Backend>(&mut self, frame: &mut Frame<B>, logs: &Vec<[String; 3]>, in_focus: bool) {
    let rows = logs
      .iter()
      .map(|log| Row::new(log.iter().map(|txt| Cell::from(txt.as_ref()).style(self.normal_style))));

    let mut blk = Block::default().borders(Borders::ALL);
    if in_focus {
      blk = blk.border_style(self.selected_style);
    }

    let t = Table::new(rows)
      .header(Row::new(vec!["   Time", "Pid", "Message"]).style(Style::default().fg(Color::Yellow)))
      .block(blk)
      .column_spacing(1)
      .highlight_style(self.selected_style)
      .highlight_symbol(">> ")
      .widths(&[Constraint::Length(23), Constraint::Length(8), Constraint::Min(10)]);

    let mut ts = TableState::default();
    ts.select(Some(self.selected));
    frame.render_stateful_widget(t, self.chunks[0], &mut ts);
  }

  pub fn render_log_message<B: Backend>(&self, frame: &mut Frame<B>, msg: &str, in_focus: bool) {
    let mut blk = Block::default().borders(Borders::ALL);
    if in_focus {
      blk = blk.border_style(self.selected_style);
    }

    let msg_disp = Paragraph::new(Text::from(msg))
      .style(Style::default().fg(Color::White))
      .block(blk)
      .alignment(Alignment::Left)
      .wrap(Wrap { trim: true });
    frame.render_widget(msg_disp, self.chunks[1]);
  }

  pub fn render_level_filter<B: Backend>(&self, frame: &mut Frame<B>, (i, n, w, e): LogLevels) {
    let mut lvls = vec![];
    if i {
      lvls.push(Span::styled("[1] INFO", Style::default().bg(Color::Gray).fg(Color::White)).into())
    } else {
      lvls.push(Span::raw("[1] INFO").into())
    }
    if n {
      lvls.push(Span::styled("[2] NOTICE", Style::default().bg(Color::Cyan).fg(Color::Black)).into())
    } else {
      lvls.push(Span::raw("[2] NOTICE").into())
    }
    if w {
      lvls.push(Span::styled("[3] WARN", Style::default().bg(Color::Yellow).fg(Color::White)).into())
    } else {
      lvls.push(Span::raw("[3] WARN").into())
    }
    if e {
      lvls.push(Span::styled("[4] ERROR", Style::default().bg(Color::Red).fg(Color::White)).into())
    } else {
      lvls.push(Span::raw("[4] ERROR").into())
    }
    let tabs = Tabs::new(lvls).block(Block::default().title(" Ellit ").borders(Borders::ALL));
    frame.render_widget(tabs, self.chunks[2]);
  }
}
