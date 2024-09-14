use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use super::words_challenge_model::WordsChallengeModel;

pub fn words_challenge_view(_challenge: &WordsChallengeModel, frame: &mut Frame) {
    let typee = Paragraph::new("Typee");

    frame.render_widget(typee, frame.area());
}
