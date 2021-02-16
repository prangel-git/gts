use std::fmt;

use super::super::abstract::environment::Environment;

// Identity of tic tac toe players
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AgentId {
    X,
    O,
}

impl fmt::Display for AgentId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AgentId::O => write!(f, "O"),
            AgentId::X => write!(f, "X"),
        }
    }
}

pub type Action = u8;

// Representation of the tic tac toe board
#[derive(Debug, Clone, Copy)]
pub struct Board {
    moves_x: u16,  // As a binary string. Puts a 1 in the positions where X moved
    moves_o: u16,  // As a binary string. Puts a 1 in the positions where Y moved
    turn: AgentId, // Player that will make the next move
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut x = self.moves_x;
        let mut o = self.moves_o;

        let mut x_pos;
        let mut o_pos;

        for _ in 0..3 {
            for _ in 0..3 {
                x_pos = (x & 1) == 1;
                o_pos = (o & 1) == 1;

                x = x >> 1;
                o = o >> 1;

                match (x_pos, o_pos) {
                    (true, false) => write!(f, "| {} |", "X").ok(),
                    (false, true) => write!(f, "| {} |", "O").ok(),
                    (false, false) => write!(f, "| {} |", " ").ok(),
                    (true, true) => write!(f, "| {} |", "?").ok(),
                };
            }
            write!(f, "\n").ok();
        }

        write! {f, "End of board"}
    }
}

impl Environment<Action, AgentId> for Board {
    fn initial_state() -> Self {
        Board {
            moves_x: 0,
            moves_o: 0,
            turn: AgentId::X,
        }
    }

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

    fn what_if(&self, a: &Action) -> Self {
        let mut board = self.clone();
        board.update(a);
        return board;
    }

    fn valid_actions(&self) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::new();

        for a in 0..9 {
            if self.is_valid(&a) {
                actions.push(a);
            } else {
            }
        }
        return actions;
    }

    fn is_valid(&self, &a: &Action) -> bool {
        let a_bounded = a <= 8;
        let x_empty = !(((self.moves_x >> a) & 1) == 1);
        let y_empty = !(((self.moves_o >> a) & 1) == 1);
        return a_bounded & x_empty & y_empty;
    }

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

    fn turn(&self) -> AgentId {
        return self.turn;
    }

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

// Checks whether one of the players has a winning position.
fn is_winning(position: u16) -> bool {
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

// Checks whether the whole board is filled
fn is_filled(board: &Board) -> bool {
    let full = 0b111111111u16;
    let fill = (board.moves_x | board.moves_o) & full;
    return fill == full;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
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
