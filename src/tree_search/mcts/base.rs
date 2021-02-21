use std::collections::HashMap;
use std::hash::Hash;

use crate::abstractions::Environment;

use super::add_value;
use super::find_terminal_value;

use super::Stored;

/// Runs montecarlo tree search in an environment.
/// The number of wins and loses at that node is stored in the cache.
/// The selection function pics an action based on the currently cached values.
pub fn mcts<Action, AgentId, T>(
    env: &T,
    agent_id: &AgentId,
    selection_fn: &dyn Fn(&T, &AgentId, &HashMap<T, Stored>) -> Option<Action>,
    cache: &mut HashMap<T, Stored>,
) -> Stored
where
    AgentId: Eq,
    T: Environment<Action, AgentId> + Copy + Eq + Hash,
{
    let value = match selection_fn(env, agent_id, cache) {
        Some(action) => {
            let next_env = env.what_if(&action);
            mcts(&next_env, agent_id, selection_fn, cache)
        }
        None => find_terminal_value(env, agent_id),
    };

    add_value(env, &value, cache);

    return value;
}
