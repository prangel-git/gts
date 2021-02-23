use crate::abstractions::Environment;

use std::collections::HashMap;
use std::hash::Hash;

use super::Dsize;
use super::DMAX;

type Stored<Action> = (f64, Dsize, Option<Action>);

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
) -> Stored<Action>
where
    Action: Copy,
    AgentId: Eq,
    T: Environment<Action, AgentId> + Copy + Clone + Eq + Hash,
{
    // Checks whether the value is stored in the cache already.
    match cache.get(env) {
        Some(stored_value) => {
            // Checks if the value was stored with at least the required depth.
            if stored_value.1 >= depth {
                return *stored_value;
            }
        }
        None => {}
    }
    if env.is_terminal() {
        // If the value is terminal, we store it with maximum depth (terminal values will always have the same reward)
        let value = (reward(env, agent_id), DMAX, None);
        cache.insert(*env, value);
        return value;
    } else if depth == 0 {
        // When we reach depth 0, we store the reward.
        let value = (reward(env, agent_id), 0, None);
        cache.insert(*env, value);
        return value;
    } else {
        let new_depth = depth - 1;
        let is_agent_turn = env.turn() == *agent_id;

        let value = if is_agent_turn {
            let init_value = (f64::NEG_INFINITY, 0, None);

            env.valid_actions()
                .iter()
                .map(|a| (a, env.what_if(a)))
                .map(|(a, env)| (a, minmax(&env, agent_id, &reward, new_depth, cache)))
                .fold(init_value, |a, (act, b)| {
                    if a.0 >= b.0 {
                        a
                    } else {
                        (b.0, b.1, Some(*act))
                    }
                })
        } else {
            let init_value = (f64::INFINITY, 0, None);

            env.valid_actions()
                .iter()
                .map(|a| (a, env.what_if(a)))
                .map(|(a, env)| (a, minmax(&env, agent_id, &reward, new_depth, cache)))
                .fold(init_value, |a, (act, b)| {
                    if a.0 <= b.0 {
                        a
                    } else {
                        (b.0, b.1, Some(*act))
                    }
                })
        };

        cache.insert(*env, value);

        return value;
    }
}
