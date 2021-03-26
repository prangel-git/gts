use std::{borrow::Borrow, fmt, u16};

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

// Ability to quickly flip the operator
impl std::ops::Not for AgentId {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            AgentId::B => AgentId::W,
            AgentId::W => AgentId::B,
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

pub fn x_y_to_tile(x: u64, y: u64) -> u64  {
    // Board of width 8
    let res = x + (y * 8);
    return res;
}

// Convert a tile square to x and y coordinates
fn tile_to_x_y(tile: u64) -> (u64, u64) {
    let x = ((tile)%8) as u64;
    let y = ((tile)/8) as u64;

    return (x, y)
}

impl Board {

    fn get_tile_at_coords(&self, x: u64, y: u64) -> Option<AgentId> {
        let tile = x_y_to_tile(x, y);

        //println!("Tile: {}", tile);

        let w_pos = ((self.moves_w >> tile) & 1) == 1;
        let b_pos = ((self.moves_b >> tile) & 1) == 1;

        match (w_pos, b_pos) {
            (true, false) => Some(AgentId::W),
            (false, true) => Some(AgentId::B),
            (false, false) => None,
            (true, true) => None,
        }
    }

    fn get_tile_at_square(&self, square: u64) -> Option<AgentId> {
        let w_pos = ((self.moves_w >> square) & 1) == 1;
        let b_pos = ((self.moves_b >> square) & 1) == 1;

        match (w_pos, b_pos) {
            (true, false) => Some(AgentId::W),
            (false, true) => Some(AgentId::B),
            (false, false) => None,
            (true, true) => None,
        }
    }

    fn flip_tiles(&mut self, x: u64, y: u64, search: AgentId) {
        let moves:[(i64, i64); 8] = [
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (1, 0),
            (1, 1),
            (1, -1),
            (0, 1),
            (0, -1),
        ];

        for (dx, dy) in &moves {
            if self.check_direction(x, y, dx, dy, search) {
                self.flip_in_dir(x, y, dx, dy, search)
            }
        }
    }

    // We know the direction being check does work. Hence we can make assumptions about
    // the tiles that are being flipped. We know that since this direction works. All tiles,
    // that are b/w the start and endpoint need to be flipped.
    fn flip_in_dir(&mut self, x: u64, y: u64, dx: &i64, dy: &i64,  search: AgentId) {
        //println!("In flip!");

        let temp_x: u64 = (x as i64 + dx) as u64;
        let temp_y: u64 = (y as i64 + dy) as u64;

        //println!("Looking for another {:?}", search);
        //println!("dx: {}, dy: {}", dx, dy);

        if temp_x > 7 {
            return;
        }

        if temp_y > 7 {
            return;
        }

        let result = self.get_tile_at_coords(temp_x, temp_y);
        //println!("{:?}", result);

        let tile = x_y_to_tile(temp_x, temp_y);
        let m = 1 << tile;
        //println!("In flip!");

        match result {
            Some(r) => {
                if r == !search {
                    match r {
                        AgentId::W => {
                            self.moves_w ^= m;
                            self.moves_b |= m;
                        },
                        AgentId::B => {
                            self.moves_b ^= m;
                            self.moves_w |= m;
                        },
                    }
                } else {
                    return;
                }
            }
            None => {return;},
        }

        self.check_direction(temp_x, temp_y, dx, dy, search);

    }

    // Assumes that the search will be none.
    // TODO: Change x and y to not be u64.
    fn check_surrounding(&self, x: u64, y: u64, search: AgentId) -> bool {
        let moves:[(i64, i64); 8] = [
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (1, 0),
            (1, 1),
            (1, -1),
            (0, 1),
            (0, -1),
        ];

        for (dx, dy) in &moves {
            let temp_x: u64 = (x as i64 + dx) as u64;
            let temp_y: u64 = (y as i64 + dy) as u64;

            if temp_x > 7 {
                return false;
            }

            if temp_y > 7 {
                return false;
            }

            let square = x_y_to_tile(temp_x, temp_y); // + 1

            let result = self.get_tile_at_square(square);

            match result {
                Some(r) => {
                    //println!("{:?}", r);
                    //println!("{:?}", search);
                    //println!("{}", r == search);
                    if r == search {
                        match search {
                            AgentId::B => {
                                let result = self.check_direction(x, y, dx, dy, AgentId::W); // Flip
                                if result {
                                    return result;
                                } else {
                                    continue;
                                }
                            }
                            AgentId::W => {
                                let result = self.check_direction(x, y, dx, dy, AgentId::B); // Flip

                                if result {
                                    return result;
                                } else {
                                    continue;
                                }
                            }
                        }
                    }
                },
                None => continue,
            }

        }

        return false;
    }

    // Maybe this can be simplified...
    // Check borders
    fn check_direction(&self, x: u64, y: u64, dx: &i64, dy: &i64, search: AgentId) -> bool {
        //println!("Looking for another {:?}", search);
        //println!("dx: {}, dy: {}", dx, dy);

        let temp_x: u64 = (x as i64 + dx) as u64;
        let temp_y: u64 = (y as i64 + dy) as u64;

        if temp_x > 7 {
            return false;
        }

        if temp_y > 7 {
            return false;
        }

        let result = self.get_tile_at_coords(temp_x, temp_y);
        //println!("{:?}", result);

        match result {
            Some(r) => {
                if r == search {
                    return true;
                } else {
                    return self.check_direction(temp_x, temp_y, dx, dy, search);
                }

            },
            None => {
                return self.check_direction(temp_x, temp_y, dx, dy, search)
            }
        }
    }

    fn board_is_full(&self) -> bool {
        for i in 0..64 {
            let tile = self.get_tile_at_square(i);
            match tile {
                Some(_) => {continue;}
                None => {return false;}
            }
        }

        return true;
    }

    pub fn count_tiles(&self) -> (i32, i32) {
        let mut white_count = 0;
        let mut black_count = 0;

        let mut copy_white = self.moves_w.clone();
        let mut copy_black = self.moves_w.clone();

        while copy_white != 0 {
            if copy_white & 1 == 1 {
                white_count += 1;
            }
            copy_white = copy_white >> 1;
        }

        while copy_black != 0 {
            if copy_black & 1 == 1 {
                black_count += 1;
            }
            copy_black = copy_black >> 1;
        }

        return (white_count, black_count);
    }

    fn two_moves_win(&self) -> bool {
        let mut b = self.clone();

        b.turn = !b.turn;
        if b.valid_actions().is_empty() {
            b.turn = !b.turn;
            if b.valid_actions().is_empty() {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
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

        // This is likely redundant but oh well?
        // TODO: Change to bit masks by default
        b.moves_w |= 1 << x_y_to_tile(3, 3);
        b.moves_b |= 1 << x_y_to_tile(3, 4);
        b.moves_w |= 1 << x_y_to_tile(4, 4);
        b.moves_b |= 1 << x_y_to_tile(4, 3);

        return b
    }

    fn update(&mut self, a: &Action) -> bool {
        // Determine if the action is valid, then be sure to flip the tiles
        if self.valid_actions().is_empty() {
            self.turn = !self.turn;
            return true;
        }

        if self.is_valid(a) {
            // //println!("This move is valid");
            let m = 1 << a;
            if self.turn == AgentId::W {
                self.moves_w |= m;
                let (x, y) = tile_to_x_y(*a);
                self.flip_tiles(x, y, AgentId::W);
            } else {
                let (x, y) = tile_to_x_y(*a);
                self.flip_tiles(x, y, AgentId::B);
                self.moves_b |= m;
            }
            self.turn = !self.turn;
            return true;
        }
        return false;
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
        //println!("Action: {}", *action);


        // Can't place a tile on one that is already there
        if self.get_tile_at_coords(x, y) != None  {
            return false
        }
        //println!("It is {:?}", self.turn);

        match self.turn {
            // These are the same but opposites
            AgentId::W => {
                // Check surrounding
                // //println!("Checking for white...");
                return self.check_surrounding(x, y, AgentId::B);
            },
            AgentId::B => {
                return self.check_surrounding(x, y, AgentId::W);
            }
        }

    }

    fn is_terminal(&self) -> bool {
        let moves = self.valid_actions();

        // Two sets of invalid moves case
        if moves.is_empty() {
            let mut board = self.clone();
            board.turn = !board.turn;

            if board.valid_actions().is_empty() {
                return true;
            }
        } else if self.board_is_full() {
            return true;
        }

        return false;
    }

    fn turn(&self) -> AgentId {
        return self.turn;
    }

    fn winner(&self) -> Option<AgentId> {
        if self.board_is_full() {
            let (white_score, black_score) = self.count_tiles();

            if white_score > black_score {
                return Some(AgentId::W)
            } else if black_score > white_score {
                return Some(AgentId::B)
            } else {
                return None;
            }
        }
        // Second case,
        else {
            if self.two_moves_win() {
                return Some(self.turn);
            } else {
                return None;
            }
        }
    }


}

// TODO: Add tests