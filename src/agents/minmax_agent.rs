use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use crate::abstractions::Agent;
use crate::abstractions::Environment;

use crate::cache::node::Cache;
use crate::tree_search::minmax;

/// A minmax agent will play by finding the best move found by the
/// minmax search with a given depth and reward function.
/// The agent caches moves previously seen
pub struct MinmaxAgent<'a, Action, AgentId, T>
where
    T: Environment<Action, AgentId>,
{
    agent_id: AgentId,
    reward: &'a dyn Fn(&T, &AgentId) -> f64,
    depth: usize,
    cache: Cache<T, Action, AgentId>,
}

/// Methods for MinmaxAgent
impl<'a, Action, AgentId, T> MinmaxAgent<'a, Action, AgentId, T>
where
    T: Environment<Action, AgentId>,
{
    pub fn new(agent_id: AgentId, reward: &'a dyn Fn(&T, &AgentId) -> f64, depth: usize) -> Self {
        MinmaxAgent {
            agent_id,
            reward,
            depth: depth + 1, // Avoiding depth 0. With depth 0, minmax does nothing.
            cache: HashMap::new(),
        }
    }
}

/// Implements an agent that runs the minmax tree search arlgorithm to produce moves.
impl<'a, Action, AgentId, T> Agent<Action, AgentId, T> for MinmaxAgent<'a, Action, AgentId, T>
where
    AgentId: Eq + Copy,
    Action: Copy,
    T: Environment<Action, AgentId> + Eq + Hash + Clone,
{
    /// Returns the agent identity in the game.
    fn identity(&self) -> AgentId {
        self.agent_id
    }

    /// Produces an action based on minmax search.
    fn action(&mut self, env: &T) -> Option<Action> {
        let env_rc = Rc::new(env.clone());
        let mut cache = HashMap::new();

        minmax(
            &env_rc,
            &self.agent_id,
            self.reward,
            self.depth,
            &mut self.cache,
            &mut cache,
        );

        self.cache = cache;

        match self.cache.get(&env_rc) {
            Some(node_ptr) => node_ptr.borrow().data.action,
            None => None,
        }
    }
}
