use games::abstract_game::environment::Environment;
use games::abstract_game::play_game;
use games::tictactoe::environment::AgentId;
use games::tictactoe::environment::Board;
use games::agents::human_agent;

fn main() {
    let mut board = Board::initial_state();

    let player_x = Box::new(human_agent::HumanPlayer::new(AgentId::X));
    let player_o = Box::new(human_agent::HumanPlayer::new(AgentId::O));
    
    let players = vec![player_x, player_o];

    play_game(&mut board, &players);

    println!("{}", board.to_string());
    
    let winner = board.winner();
    
    match winner {
        Some(x) => println!("Player {} wins.", x),
        None => println!("The game ended in a draw"),
    }
}