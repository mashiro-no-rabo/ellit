#[derive(Debug, PartialEq)]
pub enum Focus {
  App,
  MsgDisplay,
  MsgFilter,
}

/// (info, notice, warning, error)
pub type LogLevels = (bool, bool, bool, bool);

#[derive(Debug)]
pub struct App {
  pub offset: u64,
  pub count: usize,
  pub focus: Focus,
  pub log_levels: LogLevels,
}

impl Default for App {
  fn default() -> Self {
    Self {
      offset: 0,
      count: 0,
      focus: Focus::App,
      log_levels: (false, false, true, true),
    }
  }
}

impl App {
  pub fn levels_sql(&self) -> String {
    let mut lvls = vec![];
    if self.log_levels.0 {
      lvls.push("0");
    }
    if self.log_levels.1 {
      lvls.push("1");
    }
    if self.log_levels.2 {
      lvls.push("2");
    }
    if self.log_levels.3 {
      lvls.push("3");
    }
    lvls.join(",")
  }
}
