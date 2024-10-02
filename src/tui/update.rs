use color_eyre::Result;

use crate::features::words_challenge::words_challenge_update::words_challenge_update;

use super::{app_model::AppModel, app_page::AppPage, message::Message};

pub fn update(model: AppModel, msg: Message) -> Result<(AppModel, Option<Message>)> {
    if matches!(msg, Message::AppExit) {
        let finished_model = AppModel {
            app_done: true,
            ..model
        };
        return Ok((finished_model, None));
    }

    match &model.app_page {
        AppPage::WordsChallenge(challenge_model) => words_challenge_update(model, msg),
    }
}
