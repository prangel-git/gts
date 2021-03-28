type DepthType = u8;

mod stored;
pub use self::stored::Stored;

mod cache;
pub use self::cache::Cache;
