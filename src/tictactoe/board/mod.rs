mod actioniter;
mod display;
mod environment;

use self::actioniter::ActionIter;

use super::Action;
use super::AgentId;

/// Representation of the tic tac toe board
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Board {
    moves_x: u16,  // As a binary string. Puts a 1 in the positions where X moved
    moves_o: u16,  // As a binary string. Puts a 1 in the positions where Y moved
    turn: AgentId, // Player that will make the next move
}

/// Checks whether one of the players has a winning position.
fn is_winning(position: u16) -> bool {
    // Binary representation of positions that win the game.
    let winning_masks = vec![
        0b111u16,
        0b111000u16,
        0b111000000u16,
        0b1001001u16,
        0b10010010u16,
        0b100100100u16,
        0b100010001u16,
        0b1010100u16,
    ];

    for mask in winning_masks {
        if position & mask == mask {
            return true;
        }
    }
    return false;
}

/// Checks whether the whole board is filled
fn is_filled(board: &Board) -> bool {
    let full = 0b111111111u16;
    let fill = (board.moves_x | board.moves_o) & full;
    return fill == full;
}

/// Determines filled positions
fn filled_positions(board: &Board) -> u16 {
    board.moves_x | board.moves_o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abstractions::Environment;

    #[test]
    /// Plays a manual game and check that the board updates accordingly.
    fn manual_game() {
        let mut board = Board::initial_state();
        assert_eq!(board.moves_x, 0);
        assert_eq!(board.moves_o, 0);
        assert_eq!(board.turn(), AgentId::X);

        assert_eq!(board.update(&&4), true);
        assert_eq!(board.moves_x, 0b10000);
        assert_eq!(board.turn, AgentId::O);

        assert_eq!(board.update(&5), true);
        assert_eq!(board.moves_o, 0b100000);
        assert_eq!(board.turn, AgentId::X);

        assert_eq!(board.update(&0), true);
        assert_eq!(board.moves_x, 0b10001);
        assert_eq!(board.turn, AgentId::O);

        assert_eq!(board.update(&0), false);

        assert_eq!(board.update(&1), true);
        assert_eq!(board.moves_o, 0b100010);
        assert_eq!(board.turn, AgentId::X);

        assert_eq!(board.update(&8), true);
        assert_eq!(board.moves_x, 0b100010001);
        assert_eq!(board.turn, AgentId::O);

        assert_eq!(is_filled(&board), false);
        assert_eq!(is_winning(board.moves_o), false);
        assert_eq!(is_winning(board.moves_x), true);
        assert_eq!(board.is_terminal(), true);
    }
}
