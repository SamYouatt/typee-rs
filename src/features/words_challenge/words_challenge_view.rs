use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use super::character_widget::{character_widget, CharacterStyle};
use super::words_challenge_model::WordsChallengeModel;

pub fn words_challenge_view(_challenge: &WordsChallengeModel, frame: &mut Frame) {
    let typee = Paragraph::new("Typee");

    frame.render_widget(typee, frame.area());
}

impl WordsChallengeModel {
    fn render_character(self, index: usize) -> impl Widget {
    fn render_character(&self, index: usize) -> impl Widget {
        let character = self.text.chars().nth(index).unwrap();
        let is_typed = self.current_pos > index;
        let is_incorrect = self.incorrect_indices.contains(&index);

        let character_style = match (is_typed, is_incorrect) {
            (true, true) => CharacterStyle::Incorrect,
            (true, false) => CharacterStyle::Correct,
            (false, _) => CharacterStyle::NextCharacter,
        };

        character_widget(character, character_style)
    }
}
