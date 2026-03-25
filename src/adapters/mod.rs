//! # Adapters Layer
//!
//! Infrastructure implementations of ports.
//!
//! ## Adapter Types
//!
//! - **In-Memory**: For testing and development
//! - **Persistent**: For file-based caching
//! - **Shared Memory**: For cross-process caching

pub mod inmemory;
pub mod persistent;
