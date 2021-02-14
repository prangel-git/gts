use abstract_game::Environment;
use games::abstract_game;
// use games::tictactoe::environment::Action;
use games::tictactoe::environment::AgentId;
use games::tictactoe::environment::Board;
use games::agents::human_agent;

fn main() {
    let mut player_x = human_agent::HumanPlayer::new(AgentId::X);
    let mut player_o = human_agent::HumanPlayer::new(AgentId::O);
    let mut board = Board::initial_state();

    abstract_game::play_game(&mut board, &mut player_x, &mut player_o);
}