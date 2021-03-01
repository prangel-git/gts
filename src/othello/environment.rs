use std::fmt;

use super::super::abstractions::environment::Environment;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum AgentId {
    B,
    W,
}

impl fmt::Display for AgentId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            AgentId::B => write!(f, "B"),
            AgentId::W => write!(f, "W"),
        }

    }
}

// 64 bit integer to reprepsent the 64 tiles on the board.
pub type Action = u64;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Board {
    moves_b: u64,
    moves_w: u64,
    turn: AgentId
}


impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut w = self.moves_w;
        let mut b = self.moves_b;

        let mut w_pos;
        let mut b_pos;

        for _ in 0..8 {
            for _ in 0..8 {
                w_pos = (w & 1) == 1;
                b_pos = (b & 1) == 1;

                w = w >> 1;
                b = b >> 1;

                match (w_pos, b_pos) {
                    (true, false) => write!(f, "| {} |", "W").ok(),
                    (false, true) => write!(f, "| {} |", "B").ok(),
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
        let mut b: Board = Board {
            moves_b: 0,
            moves_w: 0,
            turn: AgentId::W,
        };

        b.update(&35);
        b.update(&34);
        b.update(&44);
        b.update(&45);

        return b
    }

    fn update(&mut self, a: &Action) -> bool {
        let m = 1 << a;
        if self.turn == AgentId::W {
            self.moves_w |= m;
            self.turn = AgentId::B
        } else {
            self.moves_b |= m;
            self.turn = AgentId::W
        }
        return true;
    }

    fn what_if(&self, a: &Action) -> Self {
        let mut board = self.clone();
        board.update(a);

        return board;
    }

    fn valid_actions(&self) -> Vec<Action> {

        return Vec::new();

    }

    fn is_valid(&self, action: &Action) -> bool {
        return false
    }

    fn is_terminal(&self) -> bool {
        return false;

    }

    fn turn(&self) -> AgentId {
        return self.turn;
    }

    fn winner(&self) -> Option<AgentId> {
        return None
    }


}