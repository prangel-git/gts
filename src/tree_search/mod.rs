pub type Dsize = u8; // Depth's type.
pub const DMAX: Dsize = std::u8::MAX; // Max possible depth.

mod depth_first;
pub use self::depth_first::depth_first;

mod minmax;
pub use self::minmax::minmax;

mod alphabeta;
pub use self::alphabeta::alphabeta;
