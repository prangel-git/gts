use crate::abstractions::Environment;

use std::collections::HashMap;
use std::hash::Hash;

use super::Dsize;
use super::DMAX;
use super::terminal_score;

type Stored<Action> = (f64, Option<Action>, Dsize);

/// Given a reward function, an agent identifier, and an environment, this function returns
/// an estimate of the value. To calculate that estimate, the functions visits the tree of
/// possible actions up to a given depth, and assumes that all visiting agents will take
/// actions that will maximize the reward function.
pub fn minmax<Action, AgentId, T>(
    env: &T,
    agent_id: &AgentId,
    reward: &dyn Fn(&T, &AgentId) -> f64,
    depth: Dsize,
    cache: &mut HashMap<T, Stored<Action>>,
) -> (f64, Option<Action>)
where
    Action: Copy,
    AgentId: Eq,
    T: Environment<Action, AgentId> + Clone + Eq + Hash,
{
    // Checks whether the value is stored in the cache already.
    match cache.get(env) {
        Some((stored_value, stored_action, stored_depth)) => {
            // Checks if the value was stored with at least the required depth.
            if *stored_depth >= depth {
                return (*stored_value, *stored_action);
            }
        }
        None => {}
    }
    if env.is_terminal() {
        // If the value is terminal, we store it with maximum depth (terminal values will always have the same reward)
        let stored_value = terminal_score(env, agent_id);
        cache.insert(env.clone(), (stored_value, None, DMAX));
        return (stored_value, None);
    } else if depth == 0 {
        // When we reach depth 0, we store the reward.
        let stored_value = reward(env, agent_id);
        cache.insert(env.clone(), (stored_value, None, 0));
        return (stored_value, None);
    } else {
        let new_depth = depth - 1;
        let is_agent_turn = env.turn() == *agent_id;

        let action_value = env
            .valid_actions()
            .map(|a| {
                let (score, _) = minmax(&env.what_if(&a), agent_id, &reward, new_depth, cache);
                (score, a)
            })
            .max_by(|(score0, _), (score1, _)| {
                if is_agent_turn {
                    score0
                        .partial_cmp(score1)
                        .expect("Trying to compare with NaN")
                } else {
                    score1
                        .partial_cmp(score0)
                        .expect("Trying to compare with NaN")
                }
            });

        match action_value {
            Some((value, action)) => {
                cache.insert(env.clone(), (value, Some(action), depth));
                return (value, Some(action));
            }
            None => {
                return (f64::NEG_INFINITY, None);
            }
        }
    }
}
