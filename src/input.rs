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

pub fn block_wait_action() -> Option<Action> {
  use crossterm::event::{
    read,
    Event::{Key, Resize},
    KeyCode, KeyEvent,
  };

  loop {
    match read().unwrap() {
      Key(KeyEvent {
        code: KeyCode::Char('q'),
        modifiers: _,
      }) => break Some(Action::Quit),
      Key(KeyEvent {
        code: KeyCode::Char('j'),
        modifiers: _,
      })
      | Key(KeyEvent {
        code: KeyCode::Down,
        modifiers: _,
      }) => break Some(Action::NextLog),
      Key(KeyEvent {
        code: KeyCode::Char('k'),
        modifiers: _,
      })
      | Key(KeyEvent {
        code: KeyCode::Up,
        modifiers: _,
      }) => break Some(Action::PrevLog),
      Key(KeyEvent {
        code: KeyCode::Char('l'),
        modifiers: _,
      })
      | Key(KeyEvent {
        code: KeyCode::Right,
        modifiers: _,
      }) => break Some(Action::NextPageLogs),
      Key(KeyEvent {
        code: KeyCode::Char('h'),
        modifiers: _,
      })
      | Key(KeyEvent {
        code: KeyCode::Left,
        modifiers: _,
      }) => break Some(Action::PrevPageLogs),
      Key(KeyEvent {
        code: KeyCode::Home,
        modifiers: _,
      }) => break Some(Action::TopLog),
      Key(KeyEvent {
        code: KeyCode::End,
        modifiers: _,
      }) => break Some(Action::BottomLog),
      Key(KeyEvent {
        code: KeyCode::Char('1'),
        modifiers: _,
      }) => break Some(Action::ToggleInfo),
      Key(KeyEvent {
        code: KeyCode::Char('2'),
        modifiers: _,
      }) => break Some(Action::ToggleNotice),
      Key(KeyEvent {
        code: KeyCode::Char('3'),
        modifiers: _,
      }) => break Some(Action::ToggleWarning),
      Key(KeyEvent {
        code: KeyCode::Char('4'),
        modifiers: _,
      }) => break Some(Action::ToggleError),
      Key(KeyEvent {
        code: KeyCode::Char('='),
        modifiers: _,
      })
      | Key(KeyEvent {
        code: KeyCode::Char('+'),
        modifiers: _,
      }) => break Some(Action::IncMessageHeight),
      Key(KeyEvent {
        code: KeyCode::Char('-'),
        modifiers: _,
      })
      | Key(KeyEvent {
        code: KeyCode::Char('_'),
        modifiers: _,
      }) => break Some(Action::DecMessageHeight),
      Resize(_, _) => break None,
      _ => {}
    }
  }
}
