use games::abstractions::play;
use games::abstractions::Environment;

use games::agents::alphabeta_agent::AlphabetaAgent;
use games::agents::mcts_utc_agent::MctsUctAgent;

use games::othello::Board;
use games::othello::AgentId;
// use games::othello::make_move;


// use games::tree_search::depth_first;
use games::tree_search::greedy_reward;
// use games::agents::human_agent;

/// Alpha-beta prunning vs mcts playing tic tac toe.
fn main() {
    let mut board = Board::initial_state();

    // let exploration = 2f64.sqrt();

    let mut player_w = AlphabetaAgent::new(AgentId::W, &greedy_reward, 5);
    let mut player_b = AlphabetaAgent::new(AgentId::B, &greedy_reward, 5);

    let log = play(&mut board, &mut player_w, &mut player_b);
    // let mut player_w = human_agent::HumanPlayer::new(AgentId::W);
    // let mut player_b = human_agent::HumanPlayer::new(AgentId::B);

    for (agent, mv) in log {
        println!("Player: {}, moved {}", agent, mv);
    }

    println!("{}", board.to_string());

    let winner = board.winner();

    match winner {
        Some(x) => println!("Player {} wins.", x),
        None => println!("The game ended in a draw"),
    }
}
