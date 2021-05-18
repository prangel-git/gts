mod alphabeta;
mod depth_first;
mod minmax;

use depth_first::terminal_score;

pub use alphabeta::alphabeta;
pub use depth_first::depth_first;
pub use minmax::minmax;
