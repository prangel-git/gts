mod base;
pub use self::base::mcts;

mod utils;
use self::utils::add_value;
// use self::utils::read_cache;

type Stored = (u32, u32);