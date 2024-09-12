use crossterm::event::KeyCode;

pub struct WordsChallengeModel {
    text: String,

    current_pos: usize,
}

impl WordsChallengeModel {
    pub fn generate(length: usize) -> Self {
        let text = "bongle ".repeat(length);

        Self {
            text,
            current_pos: 0,
        }
    }

    pub fn handle_input(self, keycode: KeyCode) -> Self {
        match keycode {
            KeyCode::Char(char) => self.handle_character(char),
            _ => self,
        }
    }

    fn handle_character(self, char: char) -> Self {
        Self { current_pos: self.current_pos + 1, ..self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn model_with_text(text: impl ToString) -> WordsChallengeModel {
        WordsChallengeModel {
            text: text.to_string(),
            current_pos: 0,
        }
    }

    fn model_with_test_and_pos(text: impl ToString, pos: usize) -> WordsChallengeModel {
        WordsChallengeModel {
            text: text.to_string(),
            current_pos: pos,
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
}
