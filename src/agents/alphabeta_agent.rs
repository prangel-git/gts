use std::cell::RefCell;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

use crate::abstractions::Agent;
use crate::abstractions::Environment;

use crate::cache::minmax_data::MinMaxData;
use crate::cache::node::Cache;
use crate::cache::node::Node;
use crate::tree_search::alphabeta;

/// A minmax agent plays based on a reward function and exploration of the game tree up to a given depth.
/// This agent caches previously seen environments.
pub struct AlphabetaAgent<'a, Action, AgentId, T>
where
    T: Environment<Action, AgentId>,
{
    agent_id: AgentId,
    reward: &'a dyn Fn(&T, &AgentId) -> f64,
    depth: usize,
    cache: Cache<T, Action, AgentId>,
}

/// Methods for MinmaxAgent
impl<'a, Action, AgentId, T> AlphabetaAgent<'a, Action, AgentId, T>
where
    T: Environment<Action, AgentId>,
{
    pub fn new(agent_id: AgentId, reward: &'a dyn Fn(&T, &AgentId) -> f64, depth: usize) -> Self {
        AlphabetaAgent {
            agent_id,
            reward,
            depth: depth + 1, // Avoiding depth 0. With depth 0, minmax does nothing.
            cache: Cache::new(),
        }
    }
}

/// Implements an agent that runs the minmax tree search arlgorithm to produce moves.
impl<'a, Action, AgentId, T> Agent<Action, AgentId, T> for AlphabetaAgent<'a, Action, AgentId, T>
where
    AgentId: Eq + Copy + Debug,
    Action: Copy + Debug,
    T: Environment<Action, AgentId> + Eq + Hash + Clone,
{
    /// Returns the agent identity in the game.
    fn identity(&self) -> AgentId {
        self.agent_id
    }

    /// Produces an action based on minmax search.
    fn action(&mut self, env: &T) -> Option<Action> {
        let env_rc = Rc::new(env.clone());
        let node = match self.cache.get(env) {
            Some(n) => n.clone(),
            None => Rc::new(RefCell::new(Node::new(&env_rc, MinMaxData::default()))),
        };

        self.cache.clear();

        alphabeta(
            &node,
            &self.agent_id,
            self.reward,
            self.depth,
            f64::NEG_INFINITY,
            f64::INFINITY,
            &mut self.cache,
        );

        match self.cache.get(&env_rc) {
            Some(node_ptr) => {
                println!(
                    "Agent {:?} Action {:?}, Value {:?}, CacheSize {:?}",
                    self.agent_id,
                    node_ptr.borrow().data.action,
                    node_ptr.borrow().data.value,
                    self.cache.len()
                );
                node_ptr.borrow().data.action
            }
            None => {
                println!("Agent {:?} didn't find a move", self.agent_id);
                None
            }
        }
    }
}
