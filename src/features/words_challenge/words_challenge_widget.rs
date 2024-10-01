use std::iter;

use ratatui::prelude::*;

use super::{character_widget::CharacterWidget, words_challenge_model::WordsChallengeModel};

pub struct WordsChallengeWidget<'a> {
    pub challenge_model: &'a WordsChallengeModel,
}

// Job of this widget is to fill up the given area with coloured letters from the
// model state with nice wrapping
// Where N is gonna be the number of lines accounting for text wrapping
// N and wrapping is TODO until later
impl Widget for WordsChallengeWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        // TODO: make this have as many rows as needed to allow for wrapping
        let line_layout = Layout::vertical([Constraint::Length(1)]).split(area);

        let character_constraints = iter::repeat(Constraint::Length(1)).take(self.challenge_model.text_length);
        let text_layout = Layout::horizontal(character_constraints).split(line_layout[0]);

        for letter_index in 0..self.challenge_model.text_length {
            let character_widget = CharacterWidget::get_widget_from_model(&self.challenge_model, letter_index);
            character_widget.render(text_layout[letter_index], buf);
        }
    }
}
