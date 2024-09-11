use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEventKind};

use crate::features::words_challenge::words_challenge_model::WordsChallengeModel;

use super::{app_model::AppModel, app_page::AppPage, message::Message, update::update, view::view};

pub fn run_tui() -> Result<()> {
    let mut terminal = ratatui::init();

    let words_challenge_model = WordsChallengeModel::generate(5);
    let mut app_model = AppModel {
        app_done: false,
        app_state: AppPage::WordsChallenge(words_challenge_model),
    };

    while !app_model.app_done {
        terminal.draw(|frame| view(&app_model, frame))?;

        let mut current_msg = handle_event(&app_model)?;

        while current_msg.is_some() {
            current_msg = update(&mut app_model, current_msg.unwrap())?;
        }
    }

    ratatui::try_restore()?;

    Ok(())
}

fn handle_event(app_model: &AppModel) -> Result<Option<Message>> {
    if let event::Event::Key(key) = event::read()? {
        // Handle any global key events
        if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
            return Ok(Some(Message::AppExit));
        }

        match app_model.app_state {
            AppPage::WordsChallenge(_) => todo!(),
        }
    }

    Ok(None)
}
