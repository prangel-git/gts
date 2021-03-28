use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::rc::Rc;

use crate::abstractions::Environment;

use super::{DepthType, Stored};

/// Cache structure used to store data for tree search.
pub struct Cache<Action, AgentId, T, StoredData>
where
    AgentId: Eq,
    T: Environment<Action, AgentId> + Eq + Hash,
{
    pub cache: HashMap<Rc<T>, Stored<Action, AgentId, T, StoredData>>,
}

/// Implements methods for Cache
impl<Action, AgentId, T, StoredData> Cache<Action, AgentId, T, StoredData>
where
    StoredData: Copy + Default,
    Action: Copy,
    AgentId: Eq,
    T: Environment<Action, AgentId> + Eq + Hash + Copy,
{
    /// Creates a new cache based on a particular state and a given depth.
    pub fn new(base: &Rc<T>, depth: DepthType) -> Self {
        let cache = find_descendants(base, depth);
        Cache { cache }
    }

    /// Returns a reference to a value stored in the cache
    pub fn get_or_create(&mut self, env: &Rc<T>) -> &mut Stored<Action, AgentId, T, StoredData> {
        return self
            .cache
            .entry(env.clone())
            .or_insert(Stored::new(&env.clone()));
    }

    /// Find descendants
    pub fn get_descendants(&self, env: &Rc<T>) -> HashSet<Rc<T>> {
        let mut descendants = HashSet::new();
        descendants.insert(env.clone());

        match self.cache.get(env) {
            Some(data) => {
                for (_, child) in data.get_children() {
                    for child_descendants in self.get_descendants(child) {
                        descendants.insert(child_descendants.clone());
                    }
                }
            }
            None => {}
        }

        return descendants;
    }
}

/// Finds hashmap with the descendants up to a given depth
fn find_descendants<Action, AgentId, T, StoredData>(
    env: &Rc<T>,
    depth: DepthType,
) -> HashMap<Rc<T>, Stored<Action, AgentId, T, StoredData>>
where
    StoredData: Copy + Default,
    Action: Copy,
    AgentId: Eq,
    T: Environment<Action, AgentId> + Copy + Eq + Hash,
{
    let mut cache: HashMap<Rc<T>, Stored<Action, AgentId, T, StoredData>> = HashMap::new();

    let stored: Stored<Action, AgentId, T, StoredData> = Stored::new(&env);

    if depth > 0 {
        for (_, child) in stored.get_children() {
            let child_cache = find_descendants(child, depth - 1);
            cache.extend(child_cache);
        }
    }

    cache.insert(env.clone(), stored);

    return cache;
}
