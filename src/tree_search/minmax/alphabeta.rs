use crate::abstractions::Environment;

use crate::cache::node::NodeRRMM;

use std::hash::Hash;

use super::utils::terminal_score;

const MAX_DEPTH: usize = usize::MAX >> 1; // Any depth larger than this bring its values from a terminal node.

/// Given a reward function, an agent identifier, and an environment, this function returns
/// an estimate of the value. To calculate that estimate, the functions visits the tree of
/// possible actions up to a given depth, and assumes that all visiting agents will take
/// actions that will maximize the reward function.
pub fn alphabeta<Action, AgentId, T>(
    node: &NodeRRMM<T, Action, AgentId>,
    agent_id: &AgentId,
    reward: &dyn Fn(&T, &AgentId) -> f64,
    depth: usize,
    alpha: f64,
    beta: f64,
) -> f64
where
    Action: Copy,
    AgentId: Eq,
    T: Environment<Action, AgentId> + Clone + Eq + Hash,
{
    let mut node_ptr = node.borrow_mut();

    let env = node_ptr.environment().clone();

    let is_maximizer = env.turn() == *agent_id;
    node_ptr.data.is_maximizer = is_maximizer;

    if node_ptr.data.depth >= depth {
        return node_ptr.data.value;
    } else if env.is_terminal() {
        node_ptr.data.depth = MAX_DEPTH;
        node_ptr.data.value = terminal_score(env.as_ref(), agent_id);
    } else if depth == 1 {
        node_ptr.data.depth = 1;
        node_ptr.data.value = reward(env.as_ref(), agent_id);
    } else if is_maximizer {
        node_ptr.data.depth = depth;
        node_ptr.data.value = f64::NEG_INFINITY;
        let mut next_alpha = alpha;

        node_ptr.reset();
        node_ptr.sort_children();
        while let Some((next_env, action)) = node_ptr.next() {
            let next_value = alphabeta(&next_env, agent_id, reward, depth - 1, next_alpha, beta);

            if next_value > node_ptr.data.value {
                node_ptr.data.depth = next_env.borrow().data.depth + 1;
                node_ptr.data.value = next_value;
                node_ptr.data.action = Some(action);
                next_alpha = next_value;
            };

            if next_alpha >= beta {
                break;
            }
        }
    } else {
        node_ptr.data.depth = depth;
        node_ptr.data.value = f64::INFINITY;
        let mut next_beta = beta;

        node_ptr.reset();
        node_ptr.sort_children();
        while let Some((next_env, action)) = node_ptr.next() {
            let next_value = alphabeta(&next_env, agent_id, reward, depth - 1, alpha, next_beta);

            if next_value < node_ptr.data.value {
                node_ptr.data.depth = next_env.borrow().data.depth + 1;
                node_ptr.data.value = next_value;
                node_ptr.data.action = Some(action);
                next_beta = next_value;
            }

            if next_beta <= alpha {
                break;
            }
        }
    }

    node_ptr.data.value
}
