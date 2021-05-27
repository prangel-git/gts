use crate::abstractions::Environment;
use crate::cache::node::Cache;

use crate::cache::utils::get_or_insert;

use std::hash::Hash;
use std::rc::Rc;

use super::utils::terminal_score;

/// Given a reward function, an agent identifier, and an environment, this function returns
/// an estimate of the value. To calculate that estimate, the functions visits the tree of
/// possible actions up to a given depth, and assumes that all visiting agents will take
/// actions that will maximize the reward function.
pub fn alphabeta<Action, AgentId, T>(
    env: &Rc<T>,
    agent_id: &AgentId,
    reward: &dyn Fn(&T, &AgentId) -> f64,
    depth: usize,
    alpha: f64,
    beta: f64,
    cache_old: &mut Cache<T, Action, AgentId>,
    cache_new: &mut Cache<T, Action, AgentId>,
) -> f64
where
    Action: Copy,
    AgentId: Eq,
    T: Environment<Action, AgentId> + Clone + Eq + Hash,
{
    let is_maximizer = env.turn() == *agent_id;
    let root = get_or_insert(cache_old, env);
    let mut root_ptr = root.borrow_mut();

    if root_ptr.data.depth >= depth {
    } else if root_ptr.environment().is_terminal() {
        root_ptr.data.depth = usize::MAX;
        root_ptr.data.value = terminal_score(env.as_ref(), agent_id);
    } else if depth == 1 {
        root_ptr.data.depth = 1;
        root_ptr.data.value = reward(env.as_ref(), agent_id);
    } else if is_maximizer {
        let mut value = f64::NEG_INFINITY;
        let mut next_alpha = alpha;

        root_ptr.reset();
        while let Some((next_env, action)) = root_ptr.next() {
            let next_value = alphabeta(
                &next_env,
                agent_id,
                reward,
                depth - 1,
                next_alpha,
                beta,
                cache_old,
                cache_new,
            );
            if next_value > value {
                value = next_value;
                next_alpha = value;
                root_ptr.data.value = value;
                root_ptr.data.action = Some(action);
            };

            if next_alpha >= beta {
                break;
            }
        }
        root_ptr.data.depth = depth;
    } else {
        let mut value = f64::INFINITY;
        let mut next_beta = beta;

        root_ptr.reset();
        while let Some((next_env, action)) = root_ptr.next() {
            let next_value = alphabeta(
                &next_env,
                agent_id,
                reward,
                depth - 1,
                alpha,
                next_beta,
                cache_old,
                cache_new,
            );
            if next_value < value {
                value = next_value;
                next_beta = value;
                root_ptr.data.value = value;
                root_ptr.data.action = Some(action);
            }

            if next_beta <= alpha {
                break;
            }
        }
        root_ptr.data.depth = depth;
    }

    cache_new.insert(env.clone(), root.clone());
    root_ptr.data.value
}
