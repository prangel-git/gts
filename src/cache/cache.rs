use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use crate::abstractions::Environment;

use super::Stored;

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
    
}
