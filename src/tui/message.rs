use crossterm::event::KeyCode;

pub enum Message {
    AppExit,

    ChallengeLetterInput(KeyCode),
}
