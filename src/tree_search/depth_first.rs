use super::super::abstractions::Environment;

/// Calculates the reward by going all the way down the environment tree.
pub fn depth_first<Action, AgentId, T>(env: &T, agent: &AgentId) -> f64
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
            .map(|x| depth_first(&x, agent))
            .fold(
                init_value,
                |a: f64, b: f64| if is_agent_turn { a.max(b) } else { a.min(b) },
            );

        return next_env;
    }
}