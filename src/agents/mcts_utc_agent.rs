use std::collections::HashMap;
use std::hash::Hash;

use crate::abstractions::Agent;
use crate::abstractions::Environment;

use crate::tree_search::mcts;
use crate::tree_search::uct;

/// Implements a montecarlo tree search in which the next move is piced using the
/// upper confidence bound criteria.
///
/// 'exploration' controls the trade-off between exploration and exploitation.
/// 'mc_runs' is the number of montacarlo runs in each position.
pub struct MctsUctAgent<AgentId, T>
where
    AgentId: Eq,
    T: Eq + Hash,
{
    agent_id: AgentId,
    exploration: f64,
    mc_runs: u16,
    cache: HashMap<T, (f64, u32)>,
}

/// Methods for MctsUctAgent
impl<AgentId, T> MctsUctAgent<AgentId, T>
where
    AgentId: Eq + Copy,
    T: Eq + Hash,
{
    /// creates a new montecarlo tree search agent.
    pub fn new(agent_id: AgentId, exploration: f64, mc_runs: u16) -> Self {
        MctsUctAgent {
            agent_id,
            exploration,
            mc_runs,
            cache: HashMap::new(),
        }
    }

    /// Updates the believe tree begining at the position given by env.
    fn learn<Action>(&mut self, env: &T)
    where
        Action: Copy,
        T: Environment<Action, AgentId> + Clone,
    {
        let agent_id = self.agent_id;
        let exploration = self.exploration;
        mcts(
            env,
            &agent_id,
            &|e, a, c| uct(e, a, c, exploration),
            &mut self.cache,
        );
    }
}

/// Implements an agent that runs montecarlo tree search using the ucb selection method.
impl<Action, AgentId, T> Agent<Action, AgentId, T> for MctsUctAgent<AgentId, T>
where
    AgentId: Eq + Copy,
    Action: Copy,
    T: Environment<Action, AgentId> + Eq + Hash + Clone,
{
    /// Returns the agent identity in the game.
    fn identity(&self) -> AgentId {
        return self.agent_id;
    }

    /// Produces an action based with mcts using the ucb selection method.
    fn action(&mut self, env: &T) -> Option<Action> {
        self.cache.clear();

        for _ in 0..self.mc_runs {
            self.learn(env);
        }

        // For the return, we set exploration to 0. That way we pick the best seen action.
        return uct(env, &self.agent_id, &self.cache, 0f64);
    }
}
