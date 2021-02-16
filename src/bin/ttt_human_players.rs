use games::abstractions::environment::Environment;
use games::abstractions::play_game_2players;
use games::agents::human_agent;
use games::tictactoe::environment::AgentId;
use games::tictactoe::environment::Board;

// Plays tic tac toe with two human players.
fn main() {
    let mut board = Board::initial_state();

    let mut player_x = human_agent::HumanPlayer::new(AgentId::X);
    let mut player_o = human_agent::HumanPlayer::new(AgentId::O);

    play_game_2players(&mut board, &mut player_x, &mut player_o);

    println!("{}", board.to_string());

    let winner = board.winner();

    match winner {
        Some(x) => println!("Player {} wins.", x),
        None => println!("The game ended in a draw"),
    }
}
