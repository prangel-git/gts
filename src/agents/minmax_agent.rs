use std::hash::Hash;

use crate::abstractions::Agent;
use crate::abstractions::Environment;

use crate::tree_search::minmax;

/// A minmax agent will play by finding the best move found by the
/// minmax search with a given depth and reward function.
/// The agent caches moves previously seen
pub struct MinmaxAgent<'a, AgentId, T> {
    agent_id: AgentId,
    reward: &'a dyn Fn(&T, &AgentId) -> f64,
    depth: u8,
}

/// Methods for MinmaxAgent
impl<'a, AgentId, T> MinmaxAgent<'a, AgentId, T> {
    pub fn new(agent_id: AgentId, reward: &'a dyn Fn(&T, &AgentId) -> f64, depth: u8) -> Self {
        MinmaxAgent {
            agent_id,
            reward,
            depth: depth + 1, // Avoiding depth 0. With depth 0, minmax always returns None.
        }
    }
}

/// Implements an agent that runs the minmax tree search arlgorithm to produce moves.
impl<'a, Action, AgentId, T> Agent<Action, AgentId, T> for MinmaxAgent<'a, AgentId, T>
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
        let (_, a) = minmax(env, &self.agent_id, self.reward, self.depth);
        return a;
    }
}
