use crate::abstractions::Environment;

use crate::tictactoe::{Action, AgentId};

use super::*;

/// Implementation of environment for tic tac toe board.
impl Environment<Action, AgentId> for Board {
    type ActionIter = ActionIter;

    /// Initializes an empty tic tac toe board.
    fn initial_state() -> Self {
        Board {
            moves_x: 0,
            moves_o: 0,
            turn: AgentId::X,
        }
    }

    /// Updates the board by filling the position given by action.
    /// Returns true iff the board was updated by the action.
    fn update(&mut self, a: &Action) -> bool {
        if !self.is_valid(a) {
            return false;
        } else {
            let m = 1 << a;
            if self.turn == AgentId::X {
                self.moves_x |= m;
                self.turn = AgentId::O
            } else {
                self.moves_o |= m;
                self.turn = AgentId::X
            }
            return true;
        }
    }

    /// Returns a board with what would happen if action 'a' were performed.
    fn what_if(&self, a: &Action) -> Self {
        let mut board = self.clone();
        board.update(a);
        return board;
    }

    /// Produces a list of valid actions in the current board.
    fn valid_actions(&self) -> Self::ActionIter {
        let next_action = ActionIter::new(self);
        return next_action;
    }

    /// Returns true iff the action 'a' is valid in the current board.
    fn is_valid(&self, &a: &Action) -> bool {
        let a_bounded = a <= 8;
        let x_empty = !(((self.moves_x >> a) & 1) == 1);
        let y_empty = !(((self.moves_o >> a) & 1) == 1);
        return a_bounded & x_empty & y_empty;
    }

    /// Returns true iff the board is in a terminal position.
    fn is_terminal(&self) -> bool {
        if is_winning(self.moves_x) {
            return true;
        } else if is_winning(self.moves_o) {
            return true;
        } else if is_filled(&self) {
            return true;
        } else {
            return false;
        }
    }

    /// Returns the agentId of the player for the next move.
    fn turn(&self) -> AgentId {
        return self.turn;
    }

    /// It returns Some(agentId) with agentId  of the player who won the game.
    /// If no player had won, it returns None
    fn winner(&self) -> Option<AgentId> {
        if is_winning(self.moves_x) {
            return Some(AgentId::X);
        } else if is_winning(self.moves_o) {
            return Some(AgentId::O);
        } else {
            return None;
        }
    }
}
