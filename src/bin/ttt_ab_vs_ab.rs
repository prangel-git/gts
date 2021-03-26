use games::abstractions::play;
use games::abstractions::Environment;

use games::agents::alphabeta_agent::AlphabetaAgent;
// use games::agents::alphabeta_agent::;
use games::agents::alphabeta_agent_giovanni::AlphabetaGiovanni;

use games::tictactoe::AgentId;
use games::tictactoe::Board;

use games::tree_search::depth_first;

/// Alpha-beta prunning vs alphabeta pruning playing tic tac toe.
fn main() {
    let mut board = Board::initial_state();

    // let mut player_x = AlphabetaAgent::new(AgentId::X, &depth_first, 10);
    let mut player_x = AlphabetaGiovanni::new(AgentId::X, &depth_first, 10);
    let mut player_o = AlphabetaGiovanni::new(AgentId::O, &depth_first, 10);
    // let mut player_o = AlphabetaAgent::new(AgentId::O, &depth_first, 10);

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
