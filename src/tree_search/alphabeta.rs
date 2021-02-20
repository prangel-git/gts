use super::super::abstractions::Environment;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;

use super::Dsize;
use super::DMAX;

type Stored = (f64, Dsize);

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
    cache: &mut HashMap<T, Stored>,
) -> f64
where
    AgentId: Eq,
    T: Environment<Action, AgentId> + Copy + Clone + Eq + Hash,
{
    // Checks whether the value is stored in the cache already.
    match cache.entry(*env) {
        Entry::Occupied(entry) => {
            let (stored_value, stored_depth) = *entry.get();

            // Checks if the value was stored with at least the required depth.
            if stored_depth >= depth {
                return stored_value;
            } else {
            }
        }
        Entry::Vacant(_) => {}
    }
    if env.is_terminal() {
        // If the value is terminal, we store it with maximum depth (terminal values will always have the same reward)
        let value = reward(env, agent_id);
        cache.insert(*env, (value, DMAX));
        return value;
    } else if depth == 0 {
        // When we reach depth 0, we store the reward.
        let value = reward(env, agent_id);
        cache.insert(*env, (value, 0));
        return value;
    } else {
        let next_depth = depth - 1;

        let mut next_envs = env
            .valid_actions()
            .iter()
            .map(|x| env.what_if(x))
            .collect::<Vec<T>>();

        // We sort decreasingly by the score stored in the cache.
        next_envs.sort_by(|a, b| sort_by_score(b, a, cache));

        let is_agent_turn = env.turn() == *agent_id;

        let mut value;

        if is_agent_turn {
            value = f64::NEG_INFINITY;
            let mut next_alpha = alpha;

            for next_env in next_envs {
                let this_value = alphabeta(
                    &next_env, agent_id, reward, next_depth, next_alpha, beta, cache,
                );
                value = value.max(this_value);
                next_alpha = next_alpha.max(value);

                if next_alpha >= beta {
                    break;
                }
            }
        } else {
            value = f64::INFINITY;
            let mut next_beta = beta;

            for next_env in next_envs {
                let this_value = alphabeta(
                    &next_env, agent_id, reward, next_depth, alpha, next_beta, cache,
                );
                value = value.min(this_value);
                next_beta = next_beta.min(value);

                if next_beta <= alpha {
                    break;
                }
            }
        };

        cache.insert(*env, (value, depth));

        return value;
    }
}

/// Sorts environments based on their score (as stored in a cache).
/// If an environment is not in the cache, assumes a score of NEG_INFINITY.
fn sort_by_score<T, Action, AgentId>(
    env_1: &T,
    env_2: &T,
    cache: &HashMap<T, Stored>,
) -> std::cmp::Ordering
where
    AgentId: Eq,
    T: Environment<Action, AgentId> + Eq + Hash,
{
    let v1 = match cache.get(env_1) {
        Some((value, _)) => *value,
        None => f64::NEG_INFINITY,
    };

    let v2 = match cache.get(env_2) {
        Some((value, _)) => *value,
        None => f64::NEG_INFINITY,
    };

    return v1.partial_cmp(&v2).unwrap();
}
