use crate::abstractions::Environment;

use std::{collections::HashMap, f64::NEG_INFINITY};
use std::hash::Hash;

use super::Dsize;
use super::DMAX;

type Stored<Action> = (f64, Option<Action>, Dsize);


pub fn alpha_beta_giovanni<Action, AgentId, T>(
    env: &T,
    agent_id: &AgentId,
    reward: &dyn Fn(&T, &AgentId) -> f64,
    depth: Dsize,
    alpha: f64,
    beta: f64,
    cache: &mut HashMap<T, Stored<Action>>

) -> (f64, Option<Action>)
where
    Action: Copy,
    AgentId: Eq,
    T: Environment<Action, AgentId> + Copy + Clone + Eq + Hash,
{

    match cache.get(env) {
        Some((stored_value, stored_action, stored_depth)) => {
            if *stored_depth >= depth {
                return (*stored_value, *stored_action)
            }
        }
        None => {}
    }


    // If the environment is terminal we know that the leg of the tree
    // has reached its maximum depth
    if env.is_terminal() {
        let leg_value = reward(env, agent_id);
        cache.insert(*env, (leg_value, None, DMAX));
        return (leg_value, None)
    } else if depth == 0 {
        // The function has reached its depth, get a value for that node from
        // the reward function and insert it.
        let reward_value = reward(env, agent_id);
        cache.insert(*env, (reward_value, None, 0));
        return (reward_value, None);
    } else {
        let next_depth = depth - 1;

        // Kinda stole this from prof because I thought it was clever and clean
        let mut new_environments = env
            .valid_actions()
            .iter()
            .map(|a| (*a, env.what_if(a)))
            .collect::<Vec<_>>();

        let is_turn = env.turn() == *agent_id;

        let mut ret_value;

        let mut action_choice = None;

        if is_turn {
            ret_value = f64::NEG_INFINITY;

            let mut new_alpha = alpha;

            for (action, environment) in new_environments {
                // This part is simlilar to minimax
                let (value_here, _) = alpha_beta_giovanni(
                    &environment, agent_id,reward, next_depth, new_alpha, beta, cache
                );

                // Swtich values if needed.
                if value_here > ret_value {
                    ret_value = value_here;
                    action_choice = Some(action);
                }

                // Swtich alphas if needed.
                new_alpha = new_alpha.max(ret_value);

                if new_alpha >= beta {
                    // Break because we don't even want to keep looking.
                    break;
                }

            }
        } else {
            ret_value = f64::INFINITY;

            let mut new_beta = beta;

            for (action, environment) in new_environments {

                let (value_here, _) = alpha_beta_giovanni(
                    &environment, agent_id, reward, next_depth, alpha, new_beta, cache
                );

                if value_here < ret_value {
                    ret_value = value_here;
                    action_choice = Some(action);
                }

                new_beta = new_beta.min(ret_value);


                if new_beta <= alpha {
                    // Break because we don't even want to keep looking.
                    break;
                }

            }
        }

        cache.insert(*env, (ret_value, action_choice, depth));

        return (ret_value, action_choice);
    }


}