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
    let actions = env.valid_actions();

    if actions.is_empty() {
        return None;
    } else {
        let init_action = actions[0];

        let action_wins_loses = actions
            .iter()
            .map(|x| (*x, read_cache(&env.what_if(x), cache)))
            .collect::<Vec<_>>();

        let total_simulations = action_wins_loses
            .iter()
            .fold(0u32, |a, (_, (w, l))| a + w + l) as f64;
        let exploration_numerator = exploration * total_simulations.ln().sqrt();

        let is_agent_turn = *agent_id == env.turn();
        let (best_action, _) = if is_agent_turn {
            action_wins_loses
                .iter()
                .map(|(a, (wins, loses))| (a, uct_score(*wins, *loses, exploration_numerator)))
                .fold(
                    (init_action, f64::NEG_INFINITY),
                    |(act_0, a_0), (act_1, a_1)| {
                        if a_0 < a_1 {
                            (*act_1, a_1)
                        } else {
                            (act_0, a_0)
                        }
                    },
                )
        } else {
            action_wins_loses
                .iter()
                .map(|(a, (wins, loses))| (a, uct_score(*loses, *wins, exploration_numerator)))
                .fold(
                    (init_action, f64::NEG_INFINITY),
                    |(act_0, a_0), (act_1, a_1)| {
                        if a_0 < a_1 {
                            (*act_1, a_1)
                        } else {
                            (act_0, a_0)
                        }
                    },
                )
        };

        return Some(best_action);
    }
}

fn uct_score(wins: u32, loses: u32, exploration_numerator: f64) -> f64 {
    let n_i = (wins + loses) as f64;
    let score = (wins as f64 / n_i) + exploration_numerator / n_i.sqrt();
    return score;
}
