use crate::abstractions::Agent;
use crate::abstractions::Environment;

use crate::tree_search::minmax;

/// A minmax agent plays based on a reward function and exploration of the game tree up to a given depth.
pub struct MinmaxAgent<'a, AgentId, T> {
    agent_id: AgentId,
    reward: &'a dyn Fn(&T, &AgentId) -> f64,
    depth: usize,
}

/// Methods for MinmaxAgent
impl<'a, AgentId, T> MinmaxAgent<'a, AgentId, T> {
    pub fn new(agent_id: AgentId, reward: &'a dyn Fn(&T, &AgentId) -> f64, depth: usize) -> Self {
        MinmaxAgent {
            agent_id,
            reward,
            depth: depth + 1, // Avoiding depth 0. With depth 0, minmax does nothing.
        }
    }
}

/// Implements an agent that runs the minmax tree search arlgorithm to produce moves.
impl<'a, Action, AgentId, T> Agent<Action, AgentId, T> for MinmaxAgent<'a, AgentId, T>
where
    AgentId: Eq + Copy,
    Action: Copy,
    T: Environment<Action, AgentId>,
{
    /// Returns the agent identity in the game.
    fn identity(&self) -> AgentId {
        self.agent_id
    }

    /// Produces an action based on minmax search.
    fn action(&mut self, env: &T) -> Option<Action> {
        let (_, a) = minmax(
            env,
            &self.agent_id,
            self.reward,
            self.depth,
            f64::NEG_INFINITY,
            f64::INFINITY,
        );

        a
    }
}
