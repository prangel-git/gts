use std::collections::HashMap;
use std::hash::Hash;

use crate::abstractions::Environment;

use super::add_value;

use super::Stored;

pub fn mcts<Action, AgentId, T>(
    env: &T,
    agent_id: &AgentId,
    choice_fn: &dyn Fn(&T, &AgentId, &HashMap<T, Stored>) -> Action,
    cache: &mut HashMap<T, Stored>,
) -> Stored
where
    AgentId: Eq,
    T: Environment<Action, AgentId> + Copy + Eq + Hash,
{
    if env.is_terminal() {
        let value = match env.winner() {
            Some(a) => {
                if a == *agent_id {
                    (1, 0)
                } else {
                    (0, 1)
                }
            }
            None => (1, 1),
        };
        add_value(env, &value, cache);
        return value;
    } else {
        let action = choice_fn(env, agent_id, cache);
        let next_env = env.what_if(&action);

        return mcts(&next_env, agent_id, choice_fn, cache);
    }
}

// Signature for choose action function.
fn _choose_action<Action, AgentId, T>(
    _env: &T,
    _agent_id: &AgentId,
     _cache: &HashMap<T, Stored>) -> Action
where
    AgentId: Eq,
    T: Environment<Action, AgentId> + Copy + Eq + Hash,
{
    todo!();
}