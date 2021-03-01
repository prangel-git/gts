mod alpha_beta_giovanni;
pub use self::alpha_beta_giovanni::alpha_beta_giovanni;

pub type Dsize = u8; // Depth's type.
pub const DMAX: Dsize = std::u8::MAX; // Max possible depth.
