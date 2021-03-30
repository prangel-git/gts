use std::rc::Rc;

use crate::abstractions::Environment;

/// Keeps the information stored in a cache. This information will be indexed by an environment.
pub struct Stored<Action, AgentId, T, StoredData> {
    children: Vec<(Action, Rc<T>)>,
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
        let children = env
            .valid_actions()
            .map(|action| (action, Rc::new(env.what_if(&action))))
            .collect();

        let extra = Default::default();

        let turn = env.turn();

        let is_terminal = env.is_terminal();

        Stored {
            children,
            turn,
            is_terminal,
            extra,
        }
    }

    /// Returns agent turn
    pub fn turn(&self) -> &AgentId {
        &self.turn
    }

    /// Return children
    pub fn children(&self) -> &Vec<(Action, Rc<T>)> {
        &self.children
    }

    /// Return is_terminal
    pub fn is_terminal(&self) -> bool {
        self.is_terminal
    }
}
