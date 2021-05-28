use crate::abstractions::Environment;
use crate::cache::node::Cache;

use crate::cache::node::NodeRcRefCell;

use std::hash::Hash;

use super::utils::terminal_score;

/// Given a reward function, an agent identifier, and an environment, this function returns
/// an estimate of the value. To calculate that estimate, the functions visits the tree of
/// possible actions up to a given depth, and assumes that all visiting agents will take
/// actions that will maximize the reward function.
pub fn alphabeta<Action, AgentId, T>(
    node: &NodeRcRefCell<T, Action, AgentId>,
    agent_id: &AgentId,
    reward: &dyn Fn(&T, &AgentId) -> f64,
    depth: usize,
    alpha: f64,
    beta: f64,
    cache: &mut Cache<T, Action, AgentId>,
) -> f64
where
    Action: Copy,
    AgentId: Eq,
    T: Environment<Action, AgentId> + Clone + Eq + Hash,
{
    let env = node.clone().borrow().environment().clone();

    // Reading from the cache does not work because the same two environments can be duplicated in memory.
    // This is an ugly solution to that.
    let node_tmp = match cache.get(&env) {
        Some(ptr) => {
            let mut node_ptr = node.borrow_mut();
            node_ptr.data = ptr.borrow().data;
            ptr.clone()
        }
        None => node.clone(),
    };

    let mut node_ptr = node_tmp.borrow_mut();

    // let mut node_ptr = node.borrow_mut();

    let is_maximizer = env.turn() == *agent_id;
    node_ptr.data.is_maximizer = is_maximizer;

    if node_ptr.data.depth >= depth {
        return node_ptr.data.value;
    } else if env.is_terminal() {
        node_ptr.data.depth = usize::MAX;
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
            let next_value = alphabeta(
                &next_env,
                agent_id,
                reward,
                depth - 1,
                next_alpha,
                beta,
                cache,
            );

            if next_value > node_ptr.data.value {
                node_ptr.data.value = next_value;
                next_alpha = next_value;
                node_ptr.data.action = Some(action);
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
            let next_value = alphabeta(
                &next_env,
                agent_id,
                reward,
                depth - 1,
                alpha,
                next_beta,
                cache,
            );

            if next_value < node_ptr.data.value {
                node_ptr.data.value = next_value;
                next_beta = next_value;
                node_ptr.data.action = Some(action);
            }

            if next_beta <= alpha {
                break;
            }
        }
    }

    cache.insert(env.clone(), node.clone());

    node_ptr.data.value
}
