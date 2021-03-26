use crate::{abstractions::Environment, othello::{Action, AgentId, Board}};



/// Calculates the reward by going all the way down the environment tree.
pub fn greedy_reward(env: &Board, agent: &AgentId) -> f64
{
    if env.is_terminal() {
        match env.winner() {
            Some(a) => {
                if a == *agent {
                    return 1.0;
                } else {
                    return -1.0;
                }
            }
            None => return 0.0,
        }
    } else {
        let (white_tiles, black_tiles) = env.count_tiles();
        let player = env.turn();

        let total_tiles = (white_tiles + black_tiles) as f64;

        match player {
            AgentId::B => {
                return black_tiles as f64 / total_tiles;
            }
            AgentId::W => {
                return white_tiles as f64 / total_tiles;
            }
        }
    }
}