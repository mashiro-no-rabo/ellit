use crossterm::event::{
  read,
  Event::{Key, Resize},
  KeyCode, KeyEvent,
};

pub enum Action {
  NextLog,
  PrevLog,
  NextPageLogs,
  PrevPageLogs,
  TopLog,
  BottomLog,
  ToggleInfo,
  ToggleNotice,
  ToggleWarning,
  ToggleError,
  IncMessageHeight,
  DecMessageHeight,
  ToggleFocus,
  Resize,
  Quit,
}

macro_rules! key {
  ($k:expr) => {
    Key(KeyEvent {
      code: KeyCode::Char($k),
      modifiers: _,
    })
  };
}

macro_rules! kc {
  ($k:ident) => {
    Key(KeyEvent {
      code: KeyCode::$k,
      modifiers: _,
    })
  };
}

pub fn block_wait_action() -> Action {
  loop {
    let x = match read().unwrap() {
      key!('q') => Some(Action::Quit),
      key!('j') | kc!(Down) => Some(Action::NextLog),
      key!('k') | kc!(Up) => Some(Action::PrevLog),
      key!('l') | kc!(Right) => Some(Action::NextPageLogs),
      key!('h') | kc!(Left) => Some(Action::PrevPageLogs),
      kc!(Home) => Some(Action::TopLog),
      kc!(End) => Some(Action::BottomLog),
      key!('1') => Some(Action::ToggleInfo),
      key!('2') => Some(Action::ToggleNotice),
      key!('3') => Some(Action::ToggleWarning),
      key!('4') => Some(Action::ToggleError),
      key!('=') | key!('+') => Some(Action::IncMessageHeight),
      key!('-') | key!('_') => Some(Action::DecMessageHeight),
      kc!(Tab) => Some(Action::ToggleFocus),
      Resize(_, _) => Some(Action::Resize),
      _ => None,
    };

    if x.is_some() {
      break x.unwrap();
    }
  }
}
