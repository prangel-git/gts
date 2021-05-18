use crate::abstractions::Environment;

/// Either compares score0 with score1, or score1 with score0 depending on flip.
pub fn flip_order(score0: &f64, score1: &f64, flip: bool) -> std::cmp::Ordering {
    if flip {
        score0
            .partial_cmp(score1)
            .expect("Trying to compare with NaN")
    } else {
        score1
            .partial_cmp(score0)
            .expect("Trying to compare with NaN")
    }
}

/// Finds score for terminal environments
pub fn terminal_score<Action, AgentId, T>(env: &T, agent: &AgentId) -> f64
where
    AgentId: Eq,
    T: Environment<Action, AgentId>,
{
    match env.winner() {
        Some(a) => {
            if a == *agent {
                f64::MAX
            } else {
                f64::MIN
            }
        }
        None => 0f64,
    }
}
