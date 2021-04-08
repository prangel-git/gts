use std::collections::HashMap;
use std::hash::Hash;

use crate::abstractions::Agent;
use crate::abstractions::Environment;

use crate::tree_search::alphabeta;
use crate::tree_search::Dsize;

// use super::super::tree_search::update_tree;

// TODO: This file is copied almost verbatim from the minmax_agent.
// this points out that we can abstract the two of them into one type
// of agent that would take a tree search algorithm as a parameter.

/// An agent that will play by finding the best move found by the
/// alphabeta-prunning search with a given depth and reward function.
/// The agent caches moves previously seen
pub struct AlphabetaAgent<'a, Action, AgentId, T> {
    agent_id: AgentId,
    reward: &'a dyn Fn(&T, &AgentId) -> f64,
    depth: Dsize,
    cache: HashMap<T, (f64, Option<Action>, Dsize)>,
}

/// Methods for Alphabeta
impl<'a, Action, AgentId, T> AlphabetaAgent<'a, Action, AgentId, T> {
    pub fn new(agent_id: AgentId, reward: &'a dyn Fn(&T, &AgentId) -> f64, depth: Dsize) -> Self {
        AlphabetaAgent {
            agent_id,
            reward,
            depth: depth + 1, // Preventing depth = 0. Alphabeta will always return None in that case.
            cache: HashMap::new(),
        }
    }
}

/// Implements an agent that runs alphabeta prunning tree search arlgorithm to produce moves.
impl<'a, Action, AgentId, T> Agent<Action, AgentId, T> for AlphabetaAgent<'a, Action, AgentId, T>
where
    AgentId: Eq + Copy,
    Action: Copy,
    T: Environment<Action, AgentId> + Eq + Clone + Hash,
{
    /// Returns the agent identity in the game.
    fn identity(&self) -> AgentId {
        self.agent_id
    }

    /// Produces an action based on alphabeta-pruning search.
    fn action(&mut self, env: &T) -> Option<Action> {
        // We clear the cache so it only contains information relevant to current environment.
        // update_tree(env, self.depth, &mut self.cache); // This function is being very slow.
        self.cache.clear();

        let (_, a) = alphabeta(
            env,
            &self.agent_id,
            self.reward,
            self.depth,
            f64::NEG_INFINITY,
            f64::INFINITY,
            &mut self.cache,
        );

        return a;
    }
}
