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
        (terminal_score(env, agent_id), None)
    } else if depth == 0 {
        (reward(env, agent_id), None)
    } else {
        let mut value;
        let mut action = None;

        if env.turn() == *agent_id {
            value = f64::NEG_INFINITY;
            let mut next_alpha = alpha;

            for a in env.valid_actions() {
                let (this_value, _) = alphabeta(
                    &env.what_if(&a),
                    agent_id,
                    reward,
                    depth - 1,
                    next_alpha,
                    beta,
                );

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

            for a in env.valid_actions() {
                let (this_value, _) = alphabeta(
                    &env.what_if(&a),
                    agent_id,
                    reward,
                    depth - 1,
                    alpha,
                    next_beta,
                );

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

        (value, action)
    }
}
