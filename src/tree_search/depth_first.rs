use crate::abstractions::Environment;

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
            .max_by(|a, b| {
                if is_agent_turn {
                    a.partial_cmp(&b).expect("Trying to compare with NaN")
                } else {
                    b.partial_cmp(&a).expect("Trying to compare with NaN")
                }
            });

        return next_env.unwrap();
    }
}

/// Finds score for terminal environments
fn terminal_score<Action, AgentId, T>(env: &T, agent: &AgentId) -> f64
where
    AgentId: Eq,
    T: Environment<Action, AgentId> + Copy + Clone,
{
    match env.winner() {
        Some(a) => {
            if a == *agent {
                1.0
            } else {
                -1.0
            }
        }
        None => 0.0,
    }
}
