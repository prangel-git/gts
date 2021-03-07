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
    let next_action = env.valid_actions().next();

    match next_action {
        Some(init_action) => {
            let action_score_visits = env
                .valid_actions()
                .map(|x| (x, read_cache(&env.what_if(&x), cache)))
                .collect::<Vec<_>>();

            let total_visits = action_score_visits
                .iter()
                .fold(0u32, |a, (_, (_, visits))| a + visits) as f64;

            let exploration_numerator = exploration * total_visits.ln().sqrt();

            let is_agent_turn = *agent_id == env.turn();

            let (best_action, _) = if is_agent_turn {
                action_score_visits
                    .iter()
                    .map(|(a, (score, visits))| {
                        (a, uct_score(*score, *visits, exploration_numerator))
                    })
                    .fold(
                        (init_action, f64::NEG_INFINITY),
                        |(act_0, score_0), (act_1, score_1)| {
                            if score_0 < score_1 {
                                (*act_1, score_1)
                            } else {
                                (act_0, score_0)
                            }
                        },
                    )
            } else {
                action_score_visits
                    .iter()
                    .map(|(a, (score, visits))| {
                        (a, uct_score(-(*score), *visits, exploration_numerator))
                    })
                    .fold(
                        (init_action, f64::NEG_INFINITY),
                        |(act_0, score_0), (act_1, score_1)| {
                            if score_0 < score_1 {
                                (*act_1, score_1)
                            } else {
                                (act_0, score_0)
                            }
                        },
                    )
            };

            Some(best_action)
        }
        None => None,
    }
}

fn uct_score(score: f64, visits: u32, exploration_numerator: f64) -> f64 {
    if visits == 0 {
        return exploration_numerator;
    } else {
        let n = visits as f64;
        let score = (score as f64 / n) + exploration_numerator / n.sqrt();
        return score;
    }
}
