use std::rc::Rc;

use crate::abstractions::Environment;

use super::ActionEnvironmentIter;

/// Keeps the information stored in a cache. This information will be indexed by an environment.
pub struct Stored<Action, AgentId, T, StoredData> 
where
T: Environment<Action, AgentId>
{
    action_environment: ActionEnvironmentIter<Action, AgentId, T>,
    turn: AgentId,
    is_terminal: bool,
    pub extra: StoredData,
}

/// Implements methods for the stored cache data.
impl<Action, AgentId, T, StoredData> Stored<Action, AgentId, T, StoredData>
where
    StoredData: Copy + Default,
    Action: Copy,
    AgentId: Eq,
    T: Environment<Action, AgentId>,
{
    /// Creates a new stored data.
    pub fn new(env: &Rc<T>) -> Self {
        Stored {
            action_environment: ActionEnvironmentIter::new(env),
            turn: env.turn(),
            is_terminal: env.is_terminal(),
            extra: StoredData::default(),
        }
    }

    /// Returns agent turn
    pub fn turn(&self) -> &AgentId {
        &self.turn
    }

    /// Return is_terminal
    pub fn is_terminal(&self) -> bool {
        self.is_terminal
    }

    /// Return action_environment iterator
    pub fn action_environment_iter(&self) -> &ActionEnvironmentIter<Action, AgentId, T> {
        &self.action_environment
    }
}
