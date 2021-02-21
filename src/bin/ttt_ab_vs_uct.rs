use games::abstractions::play;
use games::abstractions::Environment;

use games::agents::alphabeta_agent::AlphabetaAgent;
use games::agents::mcts_utc_agent::MctsUctAgent;

use games::tictactoe::AgentId;
use games::tictactoe::Board;

use games::tree_search::depth_first;

/// Alpha-beta prunning vs mcts playing tic tac toe.
fn main() {
    let mut board = Board::initial_state();

    let exploration = 2f64.sqrt();

    let mut player_x = MctsUctAgent::new(AgentId::X, exploration, 300);
    // let mut player_o = MctsUctAgent::new(AgentId::O, exploration, 600);
    let mut player_o = AlphabetaAgent::new(AgentId::O, &depth_first, 10);

    let log = play(&mut board, &mut player_x, &mut player_o);

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
