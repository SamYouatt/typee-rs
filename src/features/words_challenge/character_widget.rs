use ratatui::{
    buffer::Buffer,
    style::{palette::tailwind, Style},
    widgets::{Paragraph, Widget},
};

use super::words_challenge_model::WordsChallengeModel;

enum CharacterStyle {
    NextCharacter,
    Correct,
    Incorrect,
}

pub struct CharacterWidget {
    pub char: char,
    pub style: CharacterStyle,
}

impl CharacterWidget {
    pub fn get_widget_from_model(model: &WordsChallengeModel, index: usize) -> Self {
        let character = model.text.chars().nth(index).unwrap();
        let is_typed = model.current_pos > index;
        let is_incorrect = model.incorrect_indices.contains(&index);

        let character_style = match (is_typed, is_incorrect) {
            (true, true) => CharacterStyle::Incorrect,
            (true, false) => CharacterStyle::Correct,
            (false, _) => CharacterStyle::NextCharacter,
        };

        CharacterWidget {
            char: character,
            style: character_style,
        }
    }
}

impl Widget for CharacterWidget {
    fn render(self, area: ratatui::layout::Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let style = match self.style {
            CharacterStyle::NextCharacter => Style::new()
                .fg(tailwind::YELLOW.c300)
                .bg(tailwind::GRAY.c100),
            CharacterStyle::Correct => Style::new().fg(tailwind::GRAY.c800),
            CharacterStyle::Incorrect => Style::new().fg(tailwind::RED.c500),
        };

        Paragraph::new(self.char.to_string())
            .style(style)
            .render(area, buf)
    }
}
