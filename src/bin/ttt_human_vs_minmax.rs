use games::abstractions::environment::Environment;
use games::abstractions::play_game_2players;

use games::agents::human_agent;
use games::agents::minmax_agent;

use games::tictactoe::environment::AgentId;
use games::tictactoe::environment::Board;

use games::tree_search::depth_first_reward;
use minmax_agent::MinmaxAgent;

// Plays tic tac toe with two human players.
fn main() {
    let mut board = Board::initial_state();

    let mut player_x = human_agent::HumanPlayer::new(AgentId::X);
    let mut player_o = MinmaxAgent::new(AgentId::O, &depth_first_reward, 10);

    play_game_2players(&mut board, &mut player_x, &mut player_o);

    println!("{}", board.to_string());

    let winner = board.winner();

    match winner {
        Some(x) => println!("Player {} wins.", x),
        None => println!("The game ended in a draw"),
    }
}
