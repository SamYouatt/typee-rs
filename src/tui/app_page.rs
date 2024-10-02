use crate::features::words_challenge::words_challenge_model::WordsChallengeModel;

#[derive(Debug)]
pub enum AppPage {
    WordsChallenge(WordsChallengeModel),
}
