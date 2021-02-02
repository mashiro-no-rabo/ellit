use tui::{
  backend::Backend,
  layout::{Alignment, Constraint, Direction, Layout, Rect},
  style::{Color, Style},
  text::{Span, Text},
  widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState, Tabs, Wrap},
  Frame,
};

#[derive(Debug)]
pub struct Ui {
  chunks: Vec<Rect>,
  logs_table: TableState,
  normal_style: Style,
  selected_style: Style,
}

impl Ui {
  pub fn new() -> Self {
    Self {
      chunks: vec![],
      logs_table: TableState::default(),
      normal_style: Style::default().fg(Color::White),
      selected_style: Style::default().fg(Color::Green),
    }
  }

  pub fn resize_main(&mut self, msg_height: u16, area: Rect) {
    self.chunks = Layout::default()
      .direction(Direction::Vertical)
      .constraints(
        [
          Constraint::Min(5),
          Constraint::Length(msg_height),
          Constraint::Length(3),
        ]
        .as_ref(),
      )
      .split(area);
  }

  pub fn log_page_size(&self) -> u16 {
    self.chunks[0].height - 1
  }

  pub fn render_logs_table<B: Backend>(&mut self, frame: &mut Frame<B>, logs: &Vec<[String; 3]>) {
    let rows = logs
      .iter()
      .map(|log| Row::new(log.iter().map(|txt| Cell::from(txt.as_ref()).style(self.normal_style))));

    let t = Table::new(rows)
      .header(Row::new(vec!["   Time", "Pid", "Message"]).style(Style::default().fg(Color::Yellow)))
      .column_spacing(1)
      .highlight_style(self.selected_style)
      .highlight_symbol(">> ")
      .widths(&[Constraint::Length(23), Constraint::Length(8), Constraint::Min(10)]);

    frame.render_stateful_widget(t, self.chunks[0], &mut self.logs_table);
  }

  pub fn select_log(&mut self, index: usize) {
    self.logs_table.select(Some(index));
  }

  pub fn render_log_message<B: Backend>(&self, frame: &mut Frame<B>, msg: &str) {
    let msg_disp = Paragraph::new(Text::from(msg))
      .style(Style::default().fg(Color::White))
      .block(Block::default().borders(Borders::TOP))
      .alignment(Alignment::Left)
      .wrap(Wrap { trim: true });
    frame.render_widget(msg_disp, self.chunks[1]);
  }

  pub fn render_level_filter<B: Backend>(&self, frame: &mut Frame<B>, i: bool, n: bool, w: bool, e: bool) {
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
