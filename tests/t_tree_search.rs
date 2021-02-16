use games::abstractions::environment::Environment;
use games::tictactoe::environment::AgentId;
use games::tictactoe::environment::Board;
use games::tree_search::depth_first_reward;
use games::tree_search::minmax_search;

use std::collections::HashMap;

#[test]
fn minmax_sanity_check() {
    let mut board = Board::initial_state();
    let id_x = AgentId::X;
    let id_o = AgentId::O;

    board.update(&0);
    board.update(&1);
    board.update(&3);
    board.update(&4);

    let value_x = depth_first_reward(&board, &id_x);
    let value_o = depth_first_reward(&board, &id_o);

    assert_eq!(1, value_x as i16);
    assert_eq!(-1, value_o as i16);

    let mut cache = HashMap::new();

    let value_x = minmax_search(&board, &id_x, &depth_first_reward, 1, &mut cache);
    let value_o = minmax_search(&board, &id_o, &depth_first_reward, 3, &mut cache);

    assert_eq!(1, value_x as i16);
    assert_eq!(-1, value_o as i16);
}
