use std::collections::HashMap;
use std::hash::Hash;

use crate::abstractions::Environment;

type Stored = (u32, u32);

pub fn mcts<Action, AgentId, T>(
    env: &T,
    agent_id: &AgentId,
    choice_fn: &dyn Fn(&T, &HashMap<T, Stored>) -> Action,
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
        let action = choice_fn(env, cache);
        let next_env = env.what_if(&action);

        return mcts(&next_env, agent_id, choice_fn, cache);
    }
}

// Read a value from the cache. It fills with zeroes when the value is not in the cache.
fn read_cache<Action, AgentId, T>(env: &T, cache: &HashMap<T, Stored>) -> Stored
where
    AgentId: Eq,
    T: Environment<Action, AgentId> + Copy + Eq + Hash,
{
    match cache.get(env) {
        Some((w, l)) => (*w, *l),
        None => (0, 0),
    }
}

// Adds a value to the value currently in the cache.
fn add_value<Action, AgentId, T>(env: &T, (v_1, v_2): &Stored, cache: &mut HashMap<T, Stored>)
where
    AgentId: Eq,
    T: Environment<Action, AgentId> + Copy + Eq + Hash,
{
    let (mut wins, mut loses) = read_cache(env, cache);

    wins += v_1;
    loses += v_2;

    cache.insert(*env, (wins, loses));
}

// Signature for choose action function.
fn _choose_action<Action, AgentId, T>(_env: &T, _cache: &HashMap<T, Stored>) -> Action
where
    AgentId: Eq,
    T: Environment<Action, AgentId> + Copy + Eq + Hash,
{
    todo!();
}
