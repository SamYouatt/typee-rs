pub struct WordsChallengeModel {
    text: String,
}

impl WordsChallengeModel {
    pub fn generate(length: usize) -> Self {
        let text = "bongle ".repeat(length);

        Self { text }
    }
}
