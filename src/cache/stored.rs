use std::rc::Rc;

use std::marker::PhantomData;

use crate::abstractions::Environment;

/// Keeps the information stored in a cache. This information will be indexed by an environment.
pub struct Stored<Action, AgentId, T, StoredData> {
    children: Vec<(Action, Rc<T>)>,
    pub extra: StoredData,
    agent_id: PhantomData<AgentId>,
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

        Stored {
            children,
            extra,
            agent_id: PhantomData,
        }
    }

    /// Returns reference to children.
    pub fn get_children(&self) -> &Vec<(Action, Rc<T>)> {
        &self.children
    }
}
