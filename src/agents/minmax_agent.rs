use std::collections::HashMap;
use std::hash::Hash;

use super::super::abstractions::Agent;
use super::super::abstractions::Environment;

use super::super::tree_search::minmax;

/// A minmax agent will play by finding the best move found by the
/// minmax search with a given depth and reward function.
/// The agent caches moves previously seen
pub struct MinmaxAgent<'a, AgentId, T>
where
    AgentId: Eq + Hash + Copy,
    T: Eq + Hash + Copy,
{
    agent_id: AgentId,
    reward: &'a dyn Fn(&T, &AgentId) -> f64,
    depth: u8,
    cache: HashMap<(T, AgentId), (f64, u8)>,
}

/// Methods for MinmaxAgent
impl<'a, AgentId, T> MinmaxAgent<'a, AgentId, T>
where
    AgentId: Eq + Hash + Copy,
    T: Eq + Hash + Copy,
{
    pub fn new(agent_id: AgentId, reward: &'a dyn Fn(&T, &AgentId) -> f64, depth: u8) -> Self {
        MinmaxAgent {
            agent_id,
            reward,
            depth,
            cache: HashMap::new(),
        }
    }
}

/// Implements an agent that runs the minmax tree search arlgorithm to produce moves.
impl<'a, Action, AgentId, T> Agent<Action, AgentId, T> for MinmaxAgent<'a, AgentId, T>
where
    AgentId: Eq + Hash + Copy,
    Action: Copy,
    T: Environment<Action, AgentId> + Eq + Hash + Copy,
{
    /// Returns the agent identity in the game.
    fn identity(&self) -> AgentId {
        return self.agent_id;
    }

    /// Produces an action based on minmax search.
    fn action(&mut self, env: &T) -> Option<Action> {
        let actions = env.valid_actions();

        if actions.is_empty() {
            return None;
        } else {
            let mut best_action = actions[0];
            let mut best_value = f64::NEG_INFINITY;

            for action in actions {
                let current_value = minmax(
                    &env.what_if(&action),
                    &self.agent_id,
                    self.reward,
                    self.depth,
                    &mut self.cache,
                );
                if current_value > best_value {
                    best_value = current_value;
                    best_action = action;
                }
            }
            self.cache.clear();

            return Some(best_action);
        }
    }
}
