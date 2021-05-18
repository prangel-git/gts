use crate::abstractions::Environment;

use std::hash::Hash;

use super::utils::terminal_score;

/// Given a reward function, an agent identifier, and an environment, this function returns
/// an estimate of the value. To calculate that estimate, the functions visits the tree of
/// possible actions up to a given depth, and assumes that all visiting agents will take
/// actions that will maximize the reward function.
pub fn alphabeta<Action, AgentId, T>(
    env: &T,
    agent_id: &AgentId,
    reward: &dyn Fn(&T, &AgentId) -> f64,
    depth: u8,
    alpha: f64,
    beta: f64,
) -> (f64, Option<Action>)
where
    Action: Copy,
    AgentId: Eq,
    T: Environment<Action, AgentId> + Clone + Eq + Hash,
{
    if env.is_terminal() {
        // If the value is terminal, we store it with maximum depth (terminal values will always have the same reward)
        let stored_value = terminal_score(env, agent_id);
        return (stored_value, None);
    } else if depth == 0 {
        // When we reach depth 0, we store the reward.
        let stored_value = reward(env, agent_id);
        return (stored_value, None);
    } else {
        let next_depth = depth - 1;

        let next_envs = env
            .valid_actions()
            .map(|a| (a, env.what_if(&a)))
            .collect::<Vec<_>>();

        let is_agent_turn = env.turn() == *agent_id;

        let mut value;

        let mut action = None;

        if is_agent_turn {
            value = f64::NEG_INFINITY;
            let mut next_alpha = alpha;

            for (a, next_env) in next_envs {
                let (this_value, _) =
                    alphabeta(&next_env, agent_id, reward, next_depth, next_alpha, beta);

                if this_value > value {
                    value = this_value;
                    next_alpha = value;
                    action = Some(a);
                }

                if next_alpha >= beta {
                    break;
                }
            }
        } else {
            value = f64::INFINITY;
            let mut next_beta = beta;

            for (a, next_env) in next_envs {
                let (this_value, _) =
                    alphabeta(&next_env, agent_id, reward, next_depth, alpha, next_beta);

                if this_value < value {
                    value = this_value;
                    next_beta = value;
                    action = Some(a);
                }

                if next_beta <= alpha {
                    break;
                }
            }
        };

        return (value, action);
    }
}
