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
            KeyCode::Char(char) => todo!(),
            _ => self
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
        }
    }

    #[test]
    fn correct_input_advances_challenge() {
        let model = model_with_text("test");

        let result = model.handle_input(KeyCode::Char('t'));

        assert_eq!(result.current_pos, 1);
    }

    #[test]
    fn incorrect_input_advances_challenge() {
        let model = model_with_text("test");

        let result = model.handle_input(KeyCode::Char('x'));

        assert_eq!(result.current_pos, 1);
    }
}
