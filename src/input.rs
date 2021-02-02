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

pub fn block_wait_action() -> Option<Action> {
  use crossterm::event::{
    read,
    Event::{Key, Resize},
    KeyCode, KeyEvent,
  };

  loop {
    match read().unwrap() {
      key!('q') => break Some(Action::Quit),
      key!('j') | kc!(Down) => break Some(Action::NextLog),
      key!('k') | kc!(Up) => break Some(Action::PrevLog),
      key!('l') | kc!(Right) => break Some(Action::NextPageLogs),
      key!('h') | kc!(Left) => break Some(Action::PrevPageLogs),
      kc!(Home) => break Some(Action::TopLog),
      kc!(End) => break Some(Action::BottomLog),
      key!('1') => break Some(Action::ToggleInfo),
      key!('2') => break Some(Action::ToggleNotice),
      key!('3') => break Some(Action::ToggleWarning),
      key!('4') => break Some(Action::ToggleError),
      key!('=') | key!('+') => break Some(Action::IncMessageHeight),
      key!('-') | key!('_') => break Some(Action::DecMessageHeight),
      Resize(_, _) => break None,
      _ => {}
    }
  }
}
