use std::collections::HashMap;
use std::hash::Hash;

use super::super::abstractions::agent::Agent;
use super::super::abstractions::environment::Environment;

use super::super::tree_search::minmax_search;

pub struct MinmaxAgent<'a, AgentId, T>
where
    AgentId: Eq + Hash + Copy,
    T: Eq + Hash + Copy,
{
    agent_id: AgentId,
    reward: &'a dyn Fn(&T, &AgentId) -> f64,
    depth: u8,
    cache: HashMap<(T, AgentId), f64>,
}

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

impl<'a, Action, AgentId, T> Agent<Action, AgentId, T> for MinmaxAgent<'a, AgentId, T>
where
    AgentId: Eq + Hash + Copy,
    Action: Copy,
    T: Environment<Action, AgentId> + Eq + Hash + Copy,
{
    fn identity(&self) -> AgentId {
        return self.agent_id;
    }

    fn action(&mut self, env: &T) -> Action {
        let actions = env.valid_actions();

        if actions.is_empty() {
            panic!("I don't have any actions to take!");
        } else {
            let mut best_action = actions[0];
            let mut best_value = f64::NEG_INFINITY;

            for action in actions {
                let current_value = minmax_search(
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
            return best_action;
        }
    }
}
