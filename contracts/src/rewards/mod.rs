pub mod config;
pub mod storage;
pub mod storage_types;

// Re-exporting these makes them accessible as crate::rewards::UserRewards
pub use config::*;
pub use storage_types::{RewardsDataKey, UserRewards}; // Optional: re-exports config functions
