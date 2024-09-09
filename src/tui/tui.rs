use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEventKind};

use super::{app_model::AppModel, view::view};

pub fn run_tui() -> Result<()> {
    let mut terminal = ratatui::init();

    let app_model = AppModel {};

    loop {
        terminal.draw(|frame| view(&app_model, frame))?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    ratatui::try_restore()?;

    Ok(())
}
