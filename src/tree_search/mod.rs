use super::abstractions::environment::Environment;

use std::collections::HashMap;

// Given a reward function, an agent identifier, and an environment, this function returns
// an estimate of the value. To calculate that estimate, the functions visits the tree of
// possible actions up to a given depth, and assumes that all visiting agents will take
// actions that will maximize the reward function.
pub fn minmax_search<Action, AgentId, T>(
    env: &T,
    agent: &AgentId,
    reward: &dyn Fn(&T, &AgentId) -> f64,
    depth: u8,
) -> f64
where
    AgentId: Eq,
    T: Environment<Action, AgentId> + Copy + Clone,
{
    if env.is_terminal() | (depth == 0) {
        return reward(env, agent);
    } else {
        let new_depth = depth - 1;
        let actions = env.valid_actions();
        let is_agent_turn = env.turn() == *agent;

        let init_value = if is_agent_turn {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        };

        // For each action, performs the action, calculates the value of the new environment,
        // and maximizes or minimizes that value depending on whether it is the turn of the player or not.
        let next_env = actions
            .iter()
            .map(|x| env.what_if(x))
            .map(|x| minmax_search(&x, agent, &reward, new_depth))
            .fold(
                init_value,
                |a: f64, b: f64| if is_agent_turn { a.max(b) } else { a.min(b) },
            );

        return next_env;
    }
}

pub fn naive_reward<Action, AgentId, T>(env: &T, agent: &AgentId) -> f64
where
    AgentId: Eq,
    T: Environment<Action, AgentId> + Copy + Clone,
{
    if env.is_terminal() {
        match env.winner() {
            Some(a) => {
                if a == *agent {
                    return 1.0;
                } else {
                    return -1.0;
                }
            }
            None => return 0.0,
        }
    } else {
        let actions = env.valid_actions();
        let is_agent_turn = env.turn() == *agent;

        let init_value = if is_agent_turn {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        };

        // For each action, performs the action, calculates the value of the new environment,
        // and maximizes or minimizes that value depending on whether it is the turn of the player or not.
        let next_env = actions
            .iter()
            .map(|x| env.what_if(x))
            .map(|x| naive_reward(&x, agent))
            .fold(
                init_value,
                |a: f64, b: f64| if is_agent_turn { a.max(b) } else { a.min(b) },
            );

        return next_env;
    }
}
