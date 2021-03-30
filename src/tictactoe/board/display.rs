use std::fmt;

use super::Board;

/// Display trait for tic tac toe board.
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
