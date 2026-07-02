#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let scores = scores.to_vec();
        Self { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores
            .iter()
            .max_by(|score_1, score_2| score_1.cmp(score_2))
            .copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores_copy = self.scores.clone();
        scores_copy.sort();
        scores_copy.iter().rev().take(3).copied().collect()
    }
}
