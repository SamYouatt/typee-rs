use ratatui::layout::Flex;
use ratatui::prelude::*;
use ratatui::Frame;

use super::words_challenge_model::WordsChallengeModel;
use super::words_challenge_widget::WordsChallengeWidget;

// Job of this function is to take the whole frame (whole screen) and build a centered rectangle of
// 80xN pixels centered vertically and horizontally
// Where N is an appropriate height for the number of words with wrapping
// TODO: the wrapping bit
pub fn words_challenge_view(challenge: &WordsChallengeModel, frame: &mut Frame) {
    let vertical_center = Layout::vertical([Constraint::Length(1)]).flex(Flex::Center).split(frame.area())[0];
    let horizontal_center = Layout::horizontal([Constraint::Length(80)]).flex(Flex::Center).split(vertical_center);

    let challenge_widget = WordsChallengeWidget {
        challenge_model: challenge,
    };

    frame.render_widget(challenge_widget, frame.area());
}
