use crate::abstractions::Environment;

use std::collections::HashMap;
use std::hash::Hash;

use std::collections::HashSet;

use super::Dsize;

// TODO: Change these functions to be efficient. They are very slow.

/// Produces an updated copy of the original cache that only contains
/// the descendantsof the environment up to a given depth.
pub fn update_tree<Action, AgentId, T, R>(env: &T, depth: Dsize, cache: &mut HashMap<T, R>)
where
    AgentId: Eq,
    T: Environment<Action, AgentId> + Copy + Eq + Hash,
    R: Copy,
{
    let visited = find_descendants(env, depth);

    cache.retain(|key, _| visited.contains(key));
}

/// Visit positions up to a given depth
fn find_descendants<Action, AgentId, T>(env: &T, depth: Dsize) -> HashSet<T>
where
    AgentId: Eq,
    T: Environment<Action, AgentId> + Copy + Eq + Hash,
{
    let mut visited = HashSet::new();
    visited.insert(*env);

    if depth == 0 || env.is_terminal() {
        return visited;
    } else {
        let children = env
            .valid_actions()
            .iter()
            .map(|a| env.what_if(a))
            .collect::<Vec<_>>();

        for child in children {
            let current_descendants = find_descendants(&child, depth - 1);
            visited.union(&current_descendants);
        }
        return visited;
    }
}
