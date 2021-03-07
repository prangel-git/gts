use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;

use crate::abstractions::Environment;

use super::read_cache;

use super::Stored;

/// Picks an action by using the upper confidence bound criteria.
pub fn uct<Action, AgentId, T>(
    env: &T,
    agent_id: &AgentId,
    cache: &HashMap<T, Stored>,
    exploration: f64,
) -> Option<Action>
where
    Action: Copy,
    AgentId: Eq,
    T: Environment<Action, AgentId> + Copy + Eq + Hash,
{
    let total_visits: u32 = env
        .valid_actions()
        .map(|action| {
            let (_, visits) = read_cache(&env.what_if(&action), cache);
            visits
        })
        .sum();

    let exploration_numerator = exploration * (total_visits as f64).ln().sqrt();

    let is_agent_turn = *agent_id == env.turn();

    let best_action = env
        .valid_actions()
        .map(|x| (x, read_cache(&env.what_if(&x), cache)))
        .map(|(x, (score, visits))| {
            let sc = if is_agent_turn { score } else { -score };
            (x, uct_score(sc, visits, exploration_numerator))
        })
        .max_by(|(_, score0), (_, score1)| {
            if score0 < score1 {
                Ordering::Less
            } else if score0 == score1 {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        });

    match best_action {
        Some((action, _)) => Some(action),
        None => None,
    }
}

/// Calculates the uct score of an action based on the average score of and the number of visits of a node.
fn uct_score(score: f64, visits: u32, exploration_numerator: f64) -> f64 {
    if visits == 0 {
        return f64::INFINITY;
    } else {
        let n = visits as f64;
        let score = (score as f64 / n) + exploration_numerator / n.sqrt();
        return score;
    }
}
