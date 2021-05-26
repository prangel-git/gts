use crate::abstractions::Environment;

use super::utils::terminal_score;

/// Given a reward function, an agent identifier, and an environment, this function returns
/// an estimate of the value. To calculate that estimate, the functions visits the tree of
/// possible actions up to a given depth, and assumes that all visiting agents will take
/// actions that will maximize the reward function.
pub fn minmax<Action, AgentId, T>(
    env: &T,
    agent_id: &AgentId,
    reward: &dyn Fn(&T, &AgentId) -> f64,
    depth: usize,
    alpha: f64,
    beta: f64,
) -> (f64, Option<Action>)
where
    AgentId: Eq,
    T: Environment<Action, AgentId>,
{
    let is_maximizer = env.turn() == *agent_id;

    let mut value;
    let mut action = None;

    if env.is_terminal() {
        value = terminal_score(env, agent_id);
    } else if depth <= 1 {
        value = reward(env, agent_id);
    } else if is_maximizer {
        value = f64::NEG_INFINITY;
        let mut next_alpha = alpha;

        for a in env.valid_actions() {
            let (next_value, _) = minmax(
                &env.what_if(&a),
                agent_id,
                reward,
                depth - 1,
                next_alpha,
                beta,
            );
            if next_value > value {
                value = next_value;
                next_alpha = next_value;
                action = Some(a);
            };

            if next_alpha >= beta {
                break;
            }
        }
    } else {
        value = f64::INFINITY;
        let mut next_beta = beta;

        for a in env.valid_actions() {
            let (next_value, _) = minmax(
                &env.what_if(&a),
                agent_id,
                reward,
                depth - 1,
                alpha,
                next_beta,
            );
            if next_value < value {
                value = next_value;
                next_beta = next_value;
                action = Some(a);
            }

            if next_beta <= alpha {
                break;
            }
        }
    }
    (value, action)
}
