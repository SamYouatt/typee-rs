use std::{collections::HashSet, time::Instant};

use crossterm::event::KeyCode;

pub struct WordsChallengeModel {
    text: String,
    text_length: usize,
    text_word_count: usize,
    // the cursor location in the test
    current_pos: usize,
    finished: bool,
    incorrect_indices: HashSet<usize>,
    start_time: Option<Instant>,
    end_time: Option<Instant>,
    running_wpm: Vec<f32>,
}

impl WordsChallengeModel {
    pub fn generate(num_words: usize) -> Self {
        let text = "bongle ".repeat(num_words);
        let text_length = text.chars().count();
        let text_word_count = text.split_whitespace().count();

        Self {
            text,
            text_length,
            text_word_count,
            current_pos: 0,
            finished: false,
            incorrect_indices: HashSet::new(),
            start_time: None,
            end_time: None,
            running_wpm: Vec::new(),
        }
    }

    pub fn handle_input(mut self, keycode: KeyCode) -> Self {
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }

        match keycode {
            KeyCode::Char(char) => self.handle_character(char),
            KeyCode::Backspace => self.handle_backspace(),
            _ => self,
        }
    }

    // calculates the wpm of a finished test, will panic for unfinished or unstarted test
    pub fn wpm(self) -> f32 {
        if self.start_time.is_none() {
            panic!("cannot calculate wpm for unstarted challenge");
        }
        if self.end_time.is_none() {
            panic!("cannot calculate wpm for unfinished challenge");
        }

        let challenge_time_mins = self
            .end_time
            .unwrap()
            .duration_since(self.start_time.unwrap())
            .as_secs_f32()
            / 60.0;

        if challenge_time_mins <= 0.0 {
            panic!("shouldn't have test duration with no or negative time");
        }

        let wpm = self.text_word_count as f32 / challenge_time_mins;
        let rounded_wpm = (wpm * 10.0).round() / 10.0;

        rounded_wpm
    }

    // requests the challenge to poll the current wpm and store it
    pub fn poll_wpm(self) -> Self {
        todo!()
    }

    // The percentage accuracy of the test, rounded to 1 decimal place
    pub fn accuracy_percent(&self) -> f32 {
        if self.incorrect_indices.is_empty() {
            return 100.0;
        }

        let correct_letters = (self.text_length - self.incorrect_indices.len()) as f32;
        let accuracy = correct_letters / (self.text_length as f32);
        let as_percent = accuracy * 100.0;
        let rounded = (as_percent * 10.0).round() / 10.0;

        return rounded;
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
                    end_time: Some(Instant::now()),
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
        let end_time_if_finished = if is_finished {
            Some(Instant::now())
        } else {
            None
        };

        Self {
            current_pos: self.current_pos + 1,
            finished: is_finished,
            incorrect_indices,
            end_time: end_time_if_finished,
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
    use std::time::{Duration, Instant};

    use super::*;

    fn model_with_text(text: impl ToString) -> WordsChallengeModel {
        let text = text.to_string();
        let text_length = text.chars().count();
        let text_word_count = text.split_whitespace().count();
        WordsChallengeModel {
            text,
            text_length,
            text_word_count,
            current_pos: 0,
            finished: false,
            incorrect_indices: HashSet::new(),
            start_time: None,
            end_time: None,
            running_wpm: Vec::new(),
        }
    }

    fn model_with_text_and_pos(text: impl ToString, pos: usize) -> WordsChallengeModel {
        let text = text.to_string();
        let text_length = text.chars().count();
        let text_word_count = text.split_whitespace().count();
        WordsChallengeModel {
            text,
            text_length,
            text_word_count,
            current_pos: pos,
            finished: false,
            incorrect_indices: HashSet::new(),
            start_time: None,
            end_time: None,
            running_wpm: Vec::new(),
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

    #[test]
    fn challenge_should_only_start_on_first_input() {
        let model = WordsChallengeModel::generate(3);
        assert_eq!(model.start_time, None);

        let result = model.handle_input(KeyCode::Char('a'));
        assert!(result.start_time.is_some());
    }

    #[test]
    fn challenge_end_time_should_populate_at_completion() {
        let model = model_with_text("an");
        assert_eq!(model.end_time, None);

        let result = model.handle_input(KeyCode::Char('a'));
        assert_eq!(result.end_time, None);

        let result = result.handle_input(KeyCode::Char('n'));
        assert!(result.start_time.is_some());
    }

    #[test]
    fn challenge_wpm_is_correct() {
        let start_time = Instant::now();
        let end_time = start_time + Duration::from_secs(3);

        let model = WordsChallengeModel {
            text: "three words long".to_string(),
            text_length: 16,
            text_word_count: 3,
            current_pos: 15,
            finished: true,
            incorrect_indices: HashSet::new(),
            start_time: Some(start_time),
            end_time: Some(end_time),
            running_wpm: Vec::new(),
        };

        let wpm = model.wpm();
        assert_eq!(wpm, 60.0);
    }

    #[test]
    fn poll_wpm_should_calculate_wpm_and_store() {
        let now = Instant::now();
        let start_time = now - Duration::from_secs(2);
        let model = WordsChallengeModel {
            text: "five words long this text".to_string(),
            text_length: 25,
            text_word_count: 5,
            current_pos: 15,
            finished: true,
            incorrect_indices: HashSet::new(),
            start_time: Some(start_time),
            end_time: None,
            running_wpm: Vec::new(),
        };

        let result = model.poll_wpm();
        assert_eq!(result.running_wpm[0], 90.0);
    }
}
