mod base;
pub use self::base::mcts;

mod uct;
pub use self::uct::uct;

mod utils;
use self::utils::add_value;
use self::utils::find_terminal_value;
use self::utils::read_cache;

type Stored = (u32, u32);
