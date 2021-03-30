use super::*;

/// Checks whether one of the players has a winning position.
pub(super) fn is_winning(position: u16) -> bool {
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
pub(super) fn is_filled(board: &Board) -> bool {
    let full = 0b111111111u16;
    let fill = (board.moves_x | board.moves_o) & full;
    return fill == full;
}
