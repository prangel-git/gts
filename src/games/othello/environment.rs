use std::fmt;

use crate::abstractions::{Agent, Environment};

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
    turn: AgentId,
    border: u64,
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

fn make_move(x: u64, y: u64) -> u64  {
    // Board of width 8
    let res = x + (y * 8);
    return res;
}

// Convert a tile square to x and y coordinates
fn tile_to_x_y(tile: u64) -> (u64, u64) {
    let x = (tile - 1)%8;
    let y = (tile - 1)/8;

    return (x, y)
}

impl Board {

    fn get_tile_at(&self, x: u64, y: u64) -> Option<AgentId> {
        let tile = make_move(x, y);

        let w_pos = ((self.moves_w >> tile) & 1) == 1;
        let b_pos = ((self.moves_b >> tile) & 1) == 1;

        match (w_pos, b_pos) {
            (true, false) => Some(AgentId::W),
            (false, true) => Some(AgentId::B),
            (false, false) => None,
            (true, true) => None,
        }
    }
}


impl Environment<Action, AgentId> for Board {

    fn initial_state() -> Self {
        let mut b: Board = Board {
            moves_b: 0,
            moves_w: 0,
            turn: AgentId::W,
            border: 0
        };

        b.update(&make_move(3, 3));
        b.update(&make_move(3, 4));
        b.update(&make_move(4, 4));
        b.update(&make_move(4, 3));

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
        // TODO: Check if the action falls on the border first

        let mut actions = Vec::new();

        for a in 0..64 {
            if self.is_valid(&a) {
                actions.push(a);
            }
        }

        return actions;

    }

    fn is_valid(&self, action: &Action) -> bool {
        // Two things for othello to be valid.
        // 1. The move must turn over tiles.
        // 2. The move must be placed adjacent to a tile of the opposite colour
        //    - i.e. if it is white's turn then the tile must be placed against a black
        //           colour
        let (x, y) = tile_to_x_y(*action);


        // Can't place a tile on one that is already there
        if self.get_tile_at(x, y) != None {
            return false
        }

        // Check all surrounding directions
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