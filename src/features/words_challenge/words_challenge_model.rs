use std::collections::HashSet;

use crossterm::event::KeyCode;

pub struct WordsChallengeModel {
    text: String,
    text_length: usize,
    // the cursor location in the test
    current_pos: usize,
    finished: bool,
    incorrect_indices: HashSet<usize>,
}

impl WordsChallengeModel {
    pub fn generate(num_words: usize) -> Self {
        let text = "bongle ".repeat(num_words);
        let text_length = text.chars().count();

        Self {
            text,
            text_length,
            current_pos: 0,
            finished: false,
            incorrect_indices: HashSet::new(),
        }
    }

    pub fn handle_input(self, keycode: KeyCode) -> Self {
        match keycode {
            KeyCode::Char(char) => self.handle_character(char),
            KeyCode::Backspace => self.handle_backspace(),
            _ => self,
        }
    }

    pub fn accuracy_percent(&self) -> f32 {
        todo!()
    }

    fn handle_character(self, input_char: char) -> Self {
        if self.finished {
            panic!("challenge should not be handling input after finishing");
        }

        // the final letter was previously incorrect, it must be corrected or skipped
        // it will never be counted as correct
        if self.current_pos == self.text_length {
            // pressing space should end challenge
            if input_char == ' ' {
                return Self {
                    finished: true,
                    ..self
                };
            }

            // further incorrect characters should do nothing
            return self;
        }

        let expected_char = self.text.chars().nth(self.current_pos).unwrap();
        let is_correct = input_char == expected_char;

        let mut incorrect_indices = self.incorrect_indices;
        if !is_correct {
            incorrect_indices.insert(self.current_pos);
        }

        let is_finished = self.current_pos == self.text_length - 1 && is_correct;

        Self {
            current_pos: self.current_pos + 1,
            finished: is_finished,
            incorrect_indices,
            ..self
        }
    }

    fn handle_backspace(self) -> Self {
        Self {
            current_pos: self.current_pos.saturating_sub(1),
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn model_with_text(text: impl ToString) -> WordsChallengeModel {
        let text = text.to_string();
        let text_length = text.chars().count();
        WordsChallengeModel {
            text,
            text_length,
            current_pos: 0,
            finished: false,
            incorrect_indices: HashSet::new(),
        }
    }

    fn model_with_text_and_pos(text: impl ToString, pos: usize) -> WordsChallengeModel {
        let text = text.to_string();
        let text_length = text.chars().count();
        WordsChallengeModel {
            text,
            text_length,
            current_pos: pos,
            finished: false,
            incorrect_indices: HashSet::new(),
        }
    }

    #[test]
    fn correct_character_advances_challenge() {
        let model = model_with_text("test");
        let result = model.handle_input(KeyCode::Char('t'));
        assert_eq!(result.current_pos, 1);
    }

    #[test]
    fn incorrect_character_advances_challenge() {
        let model = model_with_text("test");
        let result = model.handle_input(KeyCode::Char('x'));
        assert_eq!(result.current_pos, 1);
    }

    #[test]
    fn correct_space_advances_challenge() {
        let model = model_with_text_and_pos("space test", 5);
        let result = model.handle_input(KeyCode::Char(' '));
        assert_eq!(result.current_pos, 6);
    }

    #[test]
    fn incorrect_slace_advances_challenge() {
        let model = model_with_text_and_pos("space test", 2);
        let result = model.handle_input(KeyCode::Char(' '));
        assert_eq!(result.current_pos, 3);
    }

    #[test]
    fn backspace_at_start_does_nothing() {
        let model = model_with_text("test");
        let result = model.handle_input(KeyCode::Backspace);
        assert_eq!(result.current_pos, 0);
    }

    #[test]
    fn backspace_moves_backwards() {
        let model = model_with_text_and_pos("test", 2);
        let result = model.handle_input(KeyCode::Backspace);
        assert_eq!(result.current_pos, 1);
    }

    #[test]
    fn correct_final_character_finished_challenge() {
        let model = model_with_text_and_pos("test", 3);
        let result = model.handle_input(KeyCode::Char('t'));
        assert_eq!(result.finished, true);
    }

    #[test]
    fn incorrect_final_character_does_not_finish_challenge() {
        let model = model_with_text_and_pos("test", 3);
        let result = model.handle_input(KeyCode::Char('x'));
        assert_eq!(result.current_pos, 4);
        assert_eq!(result.finished, false);
    }

    #[test]
    fn further_incorrect_characters_are_not_registered() {
        let model = model_with_text_and_pos("test", 3);
        let result = model
            .handle_input(KeyCode::Char('x'))
            .handle_input(KeyCode::Char('x'))
            .handle_input(KeyCode::Char('x'));

        assert_eq!(result.current_pos, 4);
        assert_eq!(result.finished, false);
    }

    #[test]
    fn space_after_incorrect_final_character_finishes_challenge() {
        let model = model_with_text_and_pos("test", 3);
        let result = model
            .handle_input(KeyCode::Char('x'))
            .handle_input(KeyCode::Char(' '));

        assert_eq!(result.current_pos, 4);
        assert_eq!(result.finished, true);
    }

    #[test]
    fn space_as_final_incorrect_letter_does_not_finished_challenge() {
        let model = model_with_text_and_pos("test", 3);
        let result = model.handle_input(KeyCode::Char(' '));

        assert_eq!(result.current_pos, 4);
        assert_eq!(result.finished, false);
    }

    #[test]
    #[should_panic]
    fn further_characters_after_finished_panics() {
        let model = model_with_text("ab");
        let _ = model
            .handle_input(KeyCode::Char('a'))
            .handle_input(KeyCode::Char('b'))
            .handle_input(KeyCode::Char('c'));
    }

    #[test]
    fn backspace_with_incorrect_final_character_moves_backwards() {
        let model = model_with_text_and_pos("test", 3);
        let result = model.handle_input(KeyCode::Char('x'));
        assert_eq!(result.current_pos, 4);

        let result = result.handle_input(KeyCode::Backspace);
        assert_eq!(result.current_pos, 3);
    }

    #[test]
    fn fully_correct_gives_100_percent_accuracy() {
        let model = model_with_text("dog");

        let result = model
            .handle_input(KeyCode::Char('d'))
            .handle_input(KeyCode::Char('o'))
            .handle_input(KeyCode::Char('g'));

        assert_eq!(result.accuracy_percent(), 100.0);
    }

    #[test]
    fn fully_correct_with_retypes_gives_100_percent_accuracy() {
        let model = model_with_text("dog");

        let result = model
            .handle_input(KeyCode::Char('d'))
            .handle_input(KeyCode::Char('o'))
            .handle_input(KeyCode::Backspace)
            .handle_input(KeyCode::Char('o'))
            .handle_input(KeyCode::Char('g'));

        assert_eq!(result.accuracy_percent(), 100.0);
    }

    #[test]
    fn fully_incorrect_gives_0_percent_accuracy() {
        let model = model_with_text("dog");

        let result = model
            .handle_input(KeyCode::Char('c'))
            .handle_input(KeyCode::Char('a'))
            .handle_input(KeyCode::Char('t'))
            .handle_input(KeyCode::Char(' '));

        assert_eq!(result.accuracy_percent(), 0.0);
    }

    #[test]
    fn fully_incorrect_with_retypes_gives_0_percent_accuracy() {
        let model = model_with_text("an");

        let result = model
            .handle_input(KeyCode::Char('b'))
            .handle_input(KeyCode::Backspace)
            .handle_input(KeyCode::Char('a'))
            .handle_input(KeyCode::Char('x'))
            .handle_input(KeyCode::Backspace)
            .handle_input(KeyCode::Char('n'));

        assert_eq!(result.accuracy_percent(), 0.0);
    }

    #[test]
    fn some_incorrect_gives_correct_percentage() {
        let model = model_with_text("dog");

        let result = model
            .handle_input(KeyCode::Char('d'))
            .handle_input(KeyCode::Char('a'))
            .handle_input(KeyCode::Char('g'));

        assert_eq!(result.accuracy_percent(), 66.7);
    }
}
