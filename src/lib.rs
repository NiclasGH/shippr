mod error;

pub(crate) mod command;
pub(crate) mod config;
pub(crate) mod io;

pub mod actions;

// Re-Exports
pub(crate) use error::Result;
pub use error::Error;
