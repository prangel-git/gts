use gts::abstractions::play;
use gts::abstractions::Environment;

use gts::agents::human_agent;
use gts::agents::minmax_agent::MinmaxAgent;

use gts::tictactoe::AgentId;
use gts::tictactoe::Board;

use gts::tree_search::depth_first;

/// Plays tic tac toe with a human player vs minmax player
fn main() {
    let mut board = Board::initial_state();

    let mut player_x = human_agent::HumanPlayer::new(AgentId::X);
    let mut player_o = MinmaxAgent::new(AgentId::O, &depth_first, 10);

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
