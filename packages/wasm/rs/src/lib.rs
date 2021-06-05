pub mod abort;
pub mod invoke;
pub mod msgpack;
pub mod subinvoke;

pub use msgpack::write::Write;

pub type Result = std::result::Result<(), failure::Error>;
