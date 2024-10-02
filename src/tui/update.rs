use color_eyre::Result;

use crate::features::words_challenge::words_challenge_update::words_challenge_update;

use super::{app_model::AppModel, app_page::AppPage, message::Message};

pub fn update(model: AppModel, msg: Message) -> Result<(AppModel, Option<Message>)> {
    match &model.app_page {
        AppPage::WordsChallenge(challenge_model) => words_challenge_update(model, msg),
    }
}
