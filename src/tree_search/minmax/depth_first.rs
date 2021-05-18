use crate::abstractions::Environment;

use super::utils::flip_order;
use super::utils::terminal_score;

/// Calculates the reward by going all the way down the environment tree.
pub fn depth_first<Action, AgentId, T>(env: &T, agent: &AgentId) -> f64
where
    AgentId: Eq,
    T: Environment<Action, AgentId> + Copy + Clone,
{
    if env.is_terminal() {
        return terminal_score(env, agent);
    } else {
        let actions = env.valid_actions();
        let is_agent_turn = env.turn() == *agent;

        // For each action, performs the action, calculates the value of the new environment,
        // and maximizes or minimizes that value depending on whether it is the turn of the player or not.
        let next_env = actions
            .map(|x| depth_first(&env.what_if(&x), agent))
            .max_by(|a, b| flip_order(a, b, is_agent_turn));

        return next_env.unwrap();
    }
}
