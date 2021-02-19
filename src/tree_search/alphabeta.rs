use super::super::abstractions::Environment;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;

use super::Dsize;
use super::DMAX;

type Stored = (f64, Dsize);

/// Given a reward function, an agent identifier, and an environment, this function returns
/// an estimate of the value. To calculate that estimate, the functions visits the tree of
/// possible actions up to a given depth, and assumes that all visiting agents will take
/// actions that will maximize the reward function.
pub fn alphabeta<Action, AgentId, T>(
    env: &T,
    agent_id: &AgentId,
    reward: &dyn Fn(&T, &AgentId) -> f64,
    depth: Dsize,
    cache: &mut HashMap<(T, AgentId), Stored>,
) -> f64
where
    AgentId: Eq + Hash + Copy,
    T: Environment<Action, AgentId> + Copy + Clone + Eq + Hash,
{
    // Checks whether the value is stored in the cache already.
    match cache.entry((*env, *agent_id)) {
        Entry::Occupied(entry) => {
            let (stored_value, stored_depth) = *entry.get();

            // Checks if the value was stored with at least the required depth.
            if stored_depth >= depth {
                return stored_value;
            } else {
            }
        }
        Entry::Vacant(_) => {}
    }
    if env.is_terminal() {
        // If the value is terminal, we store it with maximum depth (terminal values will always have the same reward)
        let value = reward(env, agent_id);
        cache.insert((*env, *agent_id), (value, DMAX));
        return value;
    } else if depth == 0 {
        // When we reach depth 0, we store the reward.
        let value = reward(env, agent_id);
        cache.insert((*env, *agent_id), (value, 0));
        return value;
    } else {
        let new_depth = depth - 1;
        let actions = env.valid_actions();
        let is_agent_turn = env.turn() == *agent_id;

        let init_value = if is_agent_turn {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        };

        // For each action, performs the action, calculates the value of the new environment,
        // and maximizes or minimizes that value depending on whether it is the turn of the player or not.
        let value = actions
            .iter()
            .map(|x| env.what_if(x))
            .map(|x| alphabeta(&x, agent_id, &reward, new_depth, cache))
            .fold(
                init_value,
                |a: f64, b: f64| if is_agent_turn { a.max(b) } else { a.min(b) },
            );

        cache.insert((*env, *agent_id), (value, depth));

        return value;
    }
}
