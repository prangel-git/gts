use std::cell::RefCell;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

use crate::abstractions::Agent;
use crate::abstractions::Environment;

use crate::cache::node::Node;
use crate::cache::node::NodeRRMM;
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
    root: Option<NodeRRMM<T, Action, AgentId>>,
}

/// Methods for MinmaxAgent
impl<'a, Action, AgentId, T> AlphabetaAgent<'a, Action, AgentId, T>
where
    T: Environment<Action, AgentId> + Eq + Hash + Clone,
{
    pub fn new(agent_id: AgentId, reward: &'a dyn Fn(&T, &AgentId) -> f64, depth: usize) -> Self {
        AlphabetaAgent {
            agent_id,
            reward,
            depth: depth + 1, // Avoiding depth 0. With depth 0, minmax does nothing.
            root: None,
        }
    }

    fn update_root(&mut self, env: &T) -> NodeRRMM<T, Action, AgentId> {
        let env_rc = Rc::new(env.clone());
        let current_root = self
            .root
            .clone()
            .unwrap_or(Rc::new(RefCell::new(Node::new(&env_rc))));
        let new_root = current_root
            .borrow()
            .cache_get(&env_rc)
            .unwrap_or(Rc::new(RefCell::new(Node::new(&env_rc))));

        self.root = Some(new_root.clone());
        
        new_root.clone().borrow().rebase_cache();

        new_root
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
        let new_root = self.update_root(env);

        alphabeta(
            &new_root,
            &self.agent_id,
            self.reward,
            self.depth,
            f64::NEG_INFINITY,
            f64::INFINITY,
        );

        println!(
            "Agent {:?} Action {:?}, Value {:?}, CacheSize {:?}",
            self.agent_id,
            new_root.borrow().data.action,
            new_root.borrow().data.value,
            new_root.borrow().cache_len()
        );

        let output = new_root.borrow().data.action.clone();

        output
    }
}
