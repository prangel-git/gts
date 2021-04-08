use std::collections::HashMap;
use std::hash::Hash;

mod base;
pub use self::base::mcts;

mod uct;
pub use self::uct::uct;

mod utils;
use self::utils::add_value;
use self::utils::find_terminal_value;
use self::utils::read_cache;

type Stored = (f64, u32);
type Cache<T> = HashMap<T, Stored>;
