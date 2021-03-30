use crate::abstractions::Environment;

use super::*;

/// Struct to represent current occupied elements in a board after a given action
pub struct ActionIter {
    board_state: u16, // As a binary string. Puts a 1 in the occupied possitions (starting at current)
    current: Action,
}

/// Implements occupied
impl ActionIter {
    /// Initializes structure based on a given board.
    pub fn new(board: &Board) -> Self {
        let board_state = filled_positions(board);
        let current = if board.is_terminal() { 9 } else { 0 };
        ActionIter {
            board_state,
            current,
        }
    }
}

/// Implements iterator for next action
impl Iterator for ActionIter {
    type Item = Action;

    /// Define sequence to iterate
    fn next(&mut self) -> Option<Self::Item> {
        while self.board_state & 1 == 1 {
            self.board_state = self.board_state >> 1;
            self.current += 1;
        }

        let output = if self.current > 8 {
            None
        } else {
            Some(self.current)
        };

        self.board_state = self.board_state >> 1;
        self.current += 1;

        return output;
    }
}
