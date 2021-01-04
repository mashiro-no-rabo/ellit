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
  Quit,
}

pub fn block_wait_action() -> Option<Action> {
  use crossterm::event::{read, Event, KeyCode, KeyEvent};

  loop {
    match read().unwrap() {
      Event::Key(KeyEvent {
        code: KeyCode::Char('q'),
        modifiers: _,
      }) => break Some(Action::Quit),
      Event::Key(KeyEvent {
        code: KeyCode::Char('j'),
        modifiers: _,
      })
      | Event::Key(KeyEvent {
        code: KeyCode::Down,
        modifiers: _,
      }) => break Some(Action::NextLog),
      Event::Key(KeyEvent {
        code: KeyCode::Char('k'),
        modifiers: _,
      })
      | Event::Key(KeyEvent {
        code: KeyCode::Up,
        modifiers: _,
      }) => break Some(Action::PrevLog),
      Event::Key(KeyEvent {
        code: KeyCode::Char('l'),
        modifiers: _,
      })
      | Event::Key(KeyEvent {
        code: KeyCode::Right,
        modifiers: _,
      }) => break Some(Action::NextPageLogs),
      Event::Key(KeyEvent {
        code: KeyCode::Char('h'),
        modifiers: _,
      })
      | Event::Key(KeyEvent {
        code: KeyCode::Left,
        modifiers: _,
      }) => break Some(Action::PrevPageLogs),
      Event::Key(KeyEvent {
        code: KeyCode::Home,
        modifiers: _,
      }) => break Some(Action::TopLog),
      Event::Key(KeyEvent {
        code: KeyCode::End,
        modifiers: _,
      }) => break Some(Action::BottomLog),
      Event::Key(KeyEvent {
        code: KeyCode::Char('1'),
        modifiers: _,
      }) => break Some(Action::ToggleInfo),
      Event::Key(KeyEvent {
        code: KeyCode::Char('2'),
        modifiers: _,
      }) => break Some(Action::ToggleNotice),
      Event::Key(KeyEvent {
        code: KeyCode::Char('3'),
        modifiers: _,
      }) => break Some(Action::ToggleWarning),
      Event::Key(KeyEvent {
        code: KeyCode::Char('4'),
        modifiers: _,
      }) => break Some(Action::ToggleError),
      Event::Resize(_, _) => break None,
      _ => {}
    }
  }
}
