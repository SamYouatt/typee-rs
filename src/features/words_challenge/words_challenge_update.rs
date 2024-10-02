use crate::tui::{app_model::AppModel, app_page::AppPage, message::Message};
use color_eyre::Result;

pub fn words_challenge_update(
    model: AppModel,
    message: Message,
) -> Result<(AppModel, Option<Message>)> {
    match model.app_state {
        AppPage::WordsChallenge(challenge) => match message {
            Message::ChallengeLetterInput(keycode) => {
                let new_model = AppModel {
                    app_state: AppPage::WordsChallenge(challenge.handle_challenge_input(keycode)),
                    ..model
                };
                Ok((new_model, None))
            }
            _ => Ok((
                AppModel {
                    app_state: AppPage::WordsChallenge(challenge),
                    ..model
                },
                None,
            )),
        },
        _ => panic!(
            "words challenge cannot use non words challenge models, found: {:?}",
            model.app_state
        ),
    }
}
