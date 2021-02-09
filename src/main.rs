use anyhow::{bail, Result};
use crossterm::{
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{env, io::stdout};
use tui::{backend::CrosstermBackend, Terminal};

mod app;
mod input;
mod logs;
mod ui;

use app::{App, Focus};
use input::Action;
use logs::Storage;
use ui::Ui;

fn main() -> Result<()> {
  let path = match env::args().nth(1) {
    Some(p) => p,
    None => bail!("Usage: ellit [path to .lsw file]"),
  };

  let mut store = Storage::open(&path)?;
  let mut app = App::default();

  // setup terminal
  enable_raw_mode()?;
  let mut stdout = stdout();
  execute!(stdout, EnterAlternateScreen)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;
  terminal.hide_cursor()?;
  terminal.clear()?;

  let mut ui = Ui::default();

  loop {
    let mut log_page_size = 0;

    terminal.draw(|f| {
      ui.build(f.size());

      log_page_size = ui.log_page_size() as u64;

      store.set_page_size(log_page_size);
      store.set_levels_filter(app.levels_sql());

      let logs = store.logs_table();

      ui.render_logs_table(f, &logs, app.focus == Focus::App);

      let msg = match logs.get(ui.selected) {
        Some(log) => log[2].as_ref(),
        None => "",
      };
      ui.render_log_message(f, msg, app.focus == Focus::MsgDisplay);

      ui.render_level_filter(f, app.log_levels);

      app.count = logs.len();
    })?;

    match input::block_wait_action() {
      Action::Quit => {
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        break;
      }
      Action::NextLog => {
        ui.selected += 1;
        if ui.selected >= app.count {
          ui.selected = app.count - 1
        }
      }
      Action::PrevLog => match ui.selected.checked_sub(1) {
        Some(x) => ui.selected = x,
        None => ui.selected = 0,
      },
      Action::NextPageLogs => app.offset += log_page_size,
      Action::PrevPageLogs => match app.offset.checked_sub(log_page_size) {
        Some(x) => app.offset = x,
        None => ui.selected = 0,
      },
      Action::TopLog => {
        ui.selected = 0;
      }
      Action::BottomLog => ui.selected = app.count.min(log_page_size as usize),
      Action::ToggleInfo => {
        app.log_levels.0 ^= true;
      }
      Action::ToggleNotice => {
        app.log_levels.1 ^= true;
      }
      Action::ToggleWarning => {
        app.log_levels.2 ^= true;
      }
      Action::ToggleError => {
        app.log_levels.3 ^= true;
      }
      Action::IncMessageHeight => {
        ui.message_height += 3;
      }
      Action::DecMessageHeight => {
        ui.message_height -= 3;
      }
      Action::ToggleFocus => {
        if app.focus == Focus::App {
          app.focus = Focus::MsgDisplay;
        } else {
          app.focus = Focus::App;
        }
      }
      Action::Resize => {}
    }
  }

  Ok(())
}
