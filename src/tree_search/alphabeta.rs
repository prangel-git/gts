use crate::abstractions::Environment;

use std::collections::HashMap;
use std::hash::Hash;

use super::Dsize;
use super::DMAX;

type Stored<Action> = (f64, Dsize, Option<Action>);

/// Given a reward function, an agent identifier, and an environment, this function returns
/// an estimate of the value. To calculate that estimate, the functions visits the tree of
/// possible actions up to a given depth, and assumes that all visiting agents will take
/// actions that will maximize the reward function.
pub fn alphabeta<Action, AgentId, T>(
    env: &T,
    agent_id: &AgentId,
    reward: &dyn Fn(&T, &AgentId) -> f64,
    depth: Dsize,
    alpha: f64,
    beta: f64,
    cache: &mut HashMap<T, Stored<Action>>,
) -> Stored<Action>
where
    Action: Copy,
    AgentId: Eq,
    T: Environment<Action, AgentId> + Copy + Clone + Eq + Hash,
{
    // Checks whether the value is stored in the cache already.
    match cache.get(env) {
        Some(stored_value) => {
            // Checks if the value was stored with at least the required depth.
            if stored_value.1 >= depth {
                return *stored_value;
            }
        }
        None => {}
    }
    if env.is_terminal() {
        // If the value is terminal, we store it with maximum depth (terminal values will always have the same reward)
        let value = (reward(env, agent_id), DMAX, None);
        cache.insert(*env, value);
        return value;
    } else if depth == 0 {
        // When we reach depth 0, we store the reward.
        let value = (reward(env, agent_id), 0, None);
        cache.insert(*env, value);
        return value;
    } else {
        let next_depth = depth - 1;

        let mut next_envs = env
            .valid_actions()
            .iter()
            .map(|a| (*a, env.what_if(a)))
            .collect::<Vec<_>>();

        // We sort decreasingly by the score stored in the cache.
        next_envs.sort_by(|(_, a), (_, b)| sort_by_score(b, a, cache));

        let is_agent_turn = env.turn() == *agent_id;

        let mut value;

        let mut action = None;

        if is_agent_turn {
            value = f64::NEG_INFINITY;
            let mut next_alpha = alpha;

            for (a, next_env) in next_envs {
                let (this_value, _, _) = alphabeta(
                    &next_env, agent_id, reward, next_depth, next_alpha, beta, cache,
                );

                if this_value > value {
                    value = this_value;
                    action = Some(a);
                }
                next_alpha = next_alpha.max(value);

                if next_alpha >= beta {
                    break;
                }
            }
        } else {
            value = f64::INFINITY;
            let mut next_beta = beta;

            for (a, next_env) in next_envs {
                let (this_value, _, _) = alphabeta(
                    &next_env, agent_id, reward, next_depth, alpha, next_beta, cache,
                );

                if this_value < value {
                    value = this_value;
                    action = Some(a);
                }

                value = value.min(this_value);
                next_beta = next_beta.min(value);

                if next_beta <= alpha {
                    break;
                }
            }
        };

        let stored = (value, depth, action);

        cache.insert(*env, stored);

        return stored;
    }
}

/// Sorts environments based on their score (as stored in a cache).
/// If an environment is not in the cache, assumes a score of NEG_INFINITY.
fn sort_by_score<T, Action, AgentId>(
    env_1: &T,
    env_2: &T,
    cache: &HashMap<T, Stored<Action>>,
) -> std::cmp::Ordering
where
    AgentId: Eq,
    T: Environment<Action, AgentId> + Eq + Hash,
{
    let v1 = match cache.get(env_1) {
        Some((value, _, _)) => *value,
        None => f64::NEG_INFINITY,
    };

    let v2 = match cache.get(env_2) {
        Some((value, _, _)) => *value,
        None => f64::NEG_INFINITY,
    };

    return v1.partial_cmp(&v2).unwrap();
}
