//! FundGuard application initialization. Business features are not implemented yet.

mod config;

mod bootstrap;
pub use bootstrap::{StartupError, run};
