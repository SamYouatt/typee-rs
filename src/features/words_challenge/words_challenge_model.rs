use crossterm::event::KeyCode;

pub struct WordsChallengeModel {
    text: String,
    current_pos: usize,
    finished: bool,
}

impl WordsChallengeModel {
    pub fn generate(length: usize) -> Self {
        let text = "bongle ".repeat(length);

        Self {
            text,
            current_pos: 0,
            finished: false,
        }
    }

    pub fn handle_input(self, keycode: KeyCode) -> Self {
        match keycode {
            KeyCode::Char(char) => self.handle_character(char),
            KeyCode::Backspace => self.handle_backspace(),
            _ => self,
        }
    }

    fn handle_character(self, char: char) -> Self {
        Self {
            current_pos: self.current_pos + 1,
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
        WordsChallengeModel {
            text: text.to_string(),
            current_pos: 0,
            finished: false,
        }
    }

    fn model_with_test_and_pos(text: impl ToString, pos: usize) -> WordsChallengeModel {
        WordsChallengeModel {
            text: text.to_string(),
            current_pos: pos,
            finished: false,
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
        let model = model_with_test_and_pos("space test", 5);
        let result = model.handle_input(KeyCode::Char(' '));
        assert_eq!(result.current_pos, 6);
    }

    #[test]
    fn incorrect_slace_advances_challenge() {
        let model = model_with_test_and_pos("space test", 2);
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
        let model = model_with_test_and_pos("test", 2);
        let result = model.handle_input(KeyCode::Backspace);
        assert_eq!(result.current_pos, 1);
    }

    #[test]
    fn correct_final_character_finished_challenge() {
        let model = model_with_test_and_pos("test", 3);
        let result = model.handle_input(KeyCode::Char('t'));
        assert_eq!(result.finished, true);
    }

    #[test]
    fn incorrect_final_character_does_not_finish_challenge() {
        let model = model_with_test_and_pos("test", 3);
        let result = model.handle_input(KeyCode::Char('x'));
        assert_eq!(result.current_pos, 4);
        assert_eq!(result.finished, false);
    }

    #[test]
    fn further_incorrect_characters_are_not_registered() {
        let model = model_with_test_and_pos("test", 3);
        let result = model
            .handle_input(KeyCode::Char('x'))
            .handle_input(KeyCode::Char('x'))
            .handle_input(KeyCode::Char('x'));

        assert_eq!(result.current_pos, 4);
        assert_eq!(result.finished, false);
    }

    #[test]
    fn space_after_incorrect_final_character_finishes_challenge() {
        let model = model_with_test_and_pos("test", 3);
        let result = model
            .handle_input(KeyCode::Char('x'))
            .handle_input(KeyCode::Char(' '));

        assert_eq!(result.current_pos, 4);
        assert_eq!(result.finished, true);
    }

    #[test]
    fn space_as_final_incorrect_letter_does_not_finished_challenge() {
        let model = model_with_test_and_pos("test", 3);
        let result = model.handle_input(KeyCode::Char(' '));

        assert_eq!(result.current_pos, 4);
        assert_eq!(result.finished, false);
    }

    #[test]
    fn further_characters_after_finished_does_nothing() {
        let model = model_with_text("ab");
        let result = model
            .handle_input(KeyCode::Char('a'))
            .handle_input(KeyCode::Char('b'))
            .handle_input(KeyCode::Char('c'));
        assert_eq!(result.current_pos, 1);
        assert_eq!(result.finished, true);
    }
}
