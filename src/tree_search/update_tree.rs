use super::super::abstractions::Environment;

use std::collections::HashMap;
use std::hash::Hash;

use super::Dsize;

// TODO: Change these functions to be efficient. They are very slow.

/// Produces an updated copy of the original cache that only contains
/// the descendantsof the environment up to a given depth.
pub fn update_tree<Action, AgentId, T, R>(
    env: &T,
    depth: Dsize,
    original_cache: &HashMap<T, R>,
) -> HashMap<T, R>
where
    AgentId: Eq,
    T: Environment<Action, AgentId> + Copy + Clone + Eq + Hash,
    R: Copy,
{
    let mut updated_cache = HashMap::with_capacity(original_cache.capacity());
    store_tree_recursively(env, depth, original_cache, &mut updated_cache);
    return updated_cache;
}

/// Takes the descendants (up to a given depth) of environment env, and stores their cached values
/// (found in original_cache) into an updated_cache.
fn store_tree_recursively<Action, AgentId, T, R>(
    env: &T,
    depth: Dsize,
    original_cache: &HashMap<T, R>,
    updated_cache: &mut HashMap<T, R>,
) where
    AgentId: Eq,
    T: Environment<Action, AgentId> + Copy + Clone + Eq + Hash,
    R: Copy,
{
    if depth > 0 && !env.is_terminal() {
        let children = env
            .valid_actions()
            .iter()
            .map(|x| env.what_if(x))
            .collect::<Vec<T>>();

        for child in children {
            store_tree_recursively(&child, depth - 1, original_cache, updated_cache);
        }
    }

    match original_cache.get(env) {
        Some(value) => {
            updated_cache.insert(*env, *value);
        }
        None => {}
    }
}
