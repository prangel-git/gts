pub type Dsize = u8; // Depth's type.
pub const DMAX: Dsize = std::u8::MAX; // Max possible depth.

mod depth_first;
pub use self::depth_first::depth_first;
use self::depth_first::terminal_score;

mod minmax;
pub use self::minmax::minmax;

mod alphabeta;
pub use self::alphabeta::alphabeta;

mod update_tree;
pub use self::update_tree::update_tree;

mod mcts;
pub use self::mcts::mcts;
pub use self::mcts::uct;
