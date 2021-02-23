use games::abstractions::Environment;
use games::tictactoe::AgentId;
use games::tictactoe::Board;
use games::tree_search::depth_first;
use games::tree_search::minmax;

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

    let value_x = depth_first(&board, &id_x);
    let value_o = depth_first(&board, &id_o);

    assert_eq!(1, value_x as i16);
    assert_eq!(-1, value_o as i16);

    let mut cache_x = HashMap::new();
    let mut cache_y = HashMap::new();

    let (value_x, _) = minmax(&board, &id_x, &depth_first, 1, &mut cache_x);
    let (value_o, _) = minmax(&board, &id_o, &depth_first, 3, &mut cache_y);

    assert_eq!(1, value_x as i16);
    assert_eq!(-1, value_o as i16);
}
