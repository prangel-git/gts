use crate::abstractions::Environment;

use std::{collections::HashMap, hash::Hash};

use super::Stored;

// Read a value from the cache. It fills with zeroes when the value is not in the cache.
pub fn read_cache<Action, AgentId, T>(env: &T, cache: &HashMap<T, Stored>) -> Stored
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
pub fn add_value<Action, AgentId, T>(env: &T, (v_1, v_2): &Stored, cache: &mut HashMap<T, Stored>)
where
    AgentId: Eq,
    T: Environment<Action, AgentId> + Copy + Eq + Hash,
{
    let (mut wins, mut loses) = read_cache(env, cache);

    wins += v_1;
    loses += v_2;

    cache.insert(*env, (wins, loses));
}
