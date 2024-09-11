use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEventKind};

use crate::features::words_challenge::words_challenge_model::WordsChallengeModel;

use super::{app_model::AppModel, app_page::AppPage, view::view};

pub fn run_tui() -> Result<()> {
    let mut terminal = ratatui::init();

    let words_challenge_model = WordsChallengeModel::generate(5);
    let app_model = AppModel {
        app_state: AppPage::WordsChallenge(words_challenge_model),
    };

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
