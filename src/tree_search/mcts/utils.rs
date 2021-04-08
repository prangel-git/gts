use crate::abstractions::Environment;

use super::Hash;

use super::Stored;
use super::Cache;

/// Read a value from the cache. It fills with zeroes when the value is not in the cache.
pub(super) fn read_cache<Action, AgentId, T>(
    env: &T, 
    cache: &Cache<T>,
) -> Stored
where
    AgentId: Eq,
    T: Environment<Action, AgentId> + Eq + Hash + Clone,
{
    match cache.get(env) {
        Some((score, visits)) => (*score, *visits),
        None => (0f64, 0u32),
    }
}

/// Adds a value to the value currently in the cache.
pub(super) fn add_value<Action, AgentId, T>(
    env: &T,
    (this_score, this_visits): &Stored,
    cache: &mut Cache<T>,
) where
    AgentId: Eq,
    T: Environment<Action, AgentId> + Eq + Hash + Clone,
{
    let (mut score, mut visits) = read_cache(env, cache);

    score += this_score;
    visits += this_visits;

    cache.insert(env.clone(), (score, visits));
}

/// Finds the value for a terminal action.
pub(super) fn find_terminal_value<Action, AgentId, T>(
    env: &T, 
    agent_id: &AgentId
) -> Stored
where
    AgentId: Eq,
    T: Environment<Action, AgentId>,
{
    match env.winner() {
        Some(a) => {
            if a == *agent_id {
                (1f64, 1)
            } else {
                (-1f64, 1)
            }
        }
        None => (0f64, 1),
    }
}
