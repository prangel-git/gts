use games::abstract_game::environment::Environment;
use games::abstract_game::play_game;
use games::agents::human_agent;
use games::tictactoe::environment::AgentId;
use games::tictactoe::environment::Board;

fn main() {
    let mut board = Board::initial_state();

    let mut player_x = human_agent::HumanPlayer::new(AgentId::X);
    let mut player_o = human_agent::HumanPlayer::new(AgentId::O);

    let mut players = vec![&mut player_x, &mut player_o];

    play_game(&mut board, &mut players);

    println!("{}", board.to_string());

    let winner = board.winner();

    match winner {
        Some(x) => println!("Player {} wins.", x),
        None => println!("The game ended in a draw"),
    }
}
