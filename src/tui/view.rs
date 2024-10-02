use ratatui::Frame;

use crate::features::words_challenge::words_challenge_view::words_challenge_view;

use super::app_model::AppModel;
use super::app_page::AppPage;

pub fn view(model: &AppModel, frame: &mut Frame) {
    match &model.app_page {
        AppPage::WordsChallenge(challenge) => words_challenge_view(challenge, frame),
    }
}
