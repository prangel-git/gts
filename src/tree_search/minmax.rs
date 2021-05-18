use crate::abstractions::Environment;

use std::hash::Hash;

use super::terminal_score;
use super::Dsize;

/// Given a reward function, an agent identifier, and an environment, this function returns
/// an estimate of the value. To calculate that estimate, the functions visits the tree of
/// possible actions up to a given depth, and assumes that all visiting agents will take
/// actions that will maximize the reward function.
pub fn minmax<Action, AgentId, T>(
    env: &T,
    agent_id: &AgentId,
    reward: &dyn Fn(&T, &AgentId) -> f64,
    depth: Dsize,
) -> (f64, Option<Action>)
where
    Action: Copy,
    AgentId: Eq,
    T: Environment<Action, AgentId> + Clone + Eq + Hash,
{
    if env.is_terminal() {
        let stored_value = terminal_score(env, agent_id);
        return (stored_value, None);
    } else if depth == 0 {
        let stored_value = reward(env, agent_id);
        return (stored_value, None);
    } else {
        let new_depth = depth - 1;
        let is_agent_turn = env.turn() == *agent_id;

        env.valid_actions()
            .map(|a| {
                let (score, _) = minmax(&env.what_if(&a), agent_id, &reward, new_depth);
                (score, Some(a))
            })
            .max_by(|(score0, _), (score1, _)| flip_order(score0, score1, is_agent_turn))
            .unwrap()
    }
}

/// Either compares score0 with score1, or score1 with score0 depending on flip.
fn flip_order(score0: &f64, score1: &f64, flip: bool) -> std::cmp::Ordering {
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
