use std::collections::HashMap;
use std::hash::Hash;

use super::super::abstractions::Agent;
use super::super::abstractions::Environment;

use super::super::tree_search::alphabeta;
use super::super::tree_search::Dsize;

// use super::super::tree_search::update_tree;

// TODO: This file is copied almost verbatim from the minmax_agent.
// this points out that we can abstract the two of them into one type
// of agent that would take a tree search algorithm as a parameter.

/// An agent that will play by finding the best move found by the
/// alphabeta-prunning search with a given depth and reward function.
/// The agent caches moves previously seen
pub struct AlphabetaAgent<'a, AgentId, T>
where
    AgentId: Eq,
    T: Eq + Hash + Copy,
{
    agent_id: AgentId,
    reward: &'a dyn Fn(&T, &AgentId) -> f64,
    depth: Dsize,
    cache: HashMap<T, (f64, Dsize)>,
}

/// Methods for Alphabeta
impl<'a, AgentId, T> AlphabetaAgent<'a, AgentId, T>
where
    AgentId: Eq,
    T: Eq + Hash + Copy,
{
    pub fn new(agent_id: AgentId, reward: &'a dyn Fn(&T, &AgentId) -> f64, depth: Dsize) -> Self {
        AlphabetaAgent {
            agent_id,
            reward,
            depth,
            cache: HashMap::new(),
        }
    }
}

/// Implements an agent that runs alphabeta prunning tree search arlgorithm to produce moves.
impl<'a, Action, AgentId, T> Agent<Action, AgentId, T> for AlphabetaAgent<'a, AgentId, T>
where
    AgentId: Eq + Copy,
    Action: Copy,
    T: Environment<Action, AgentId> + Eq + Hash + Copy,
{
    /// Returns the agent identity in the game.
    fn identity(&self) -> AgentId {
        return self.agent_id;
    }

    /// Produces an action based on alphabeta-pruning search.
    fn action(&mut self, env: &T) -> Option<Action> {
        // We clear the cache so it only contains information relevant to current environment.
        // self.cache = update_tree(env, self.depth, &mut self.cache); // This function is being very slow.
        self.cache.clear();

        let actions = env.valid_actions();

        if actions.is_empty() {
            return None;
        } else {
            let mut best_action = actions[0];
            let mut best_value = f64::NEG_INFINITY;

            for action in actions {
                let current_value = alphabeta(
                    &env.what_if(&action),
                    &self.agent_id,
                    self.reward,
                    self.depth,
                    f64::NEG_INFINITY,
                    f64::INFINITY,
                    &mut self.cache,
                );
                if current_value > best_value {
                    best_value = current_value;
                    best_action = action;
                }
            }

            return Some(best_action);
        }
    }
}
