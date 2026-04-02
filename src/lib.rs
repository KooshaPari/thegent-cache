//! # thegent-cache
//!
//! Multi-tier caching library for agent orchestration.
//!
//! ## Architecture
//!
//! This crate follows **Hexagonal Architecture** (Ports & Adapters) with **Clean Architecture** layers.
//!
//! ## xDD Methodologies Applied
//!
//! - **TDD**: Tests written first
//! - **DDD**: Bounded context for caching
//! - **SOLID**: Single responsibility per module
//! - **CQRS**: Separate command and query interfaces
//! - **EDA**: Domain events for cache operations

pub mod adapters;
pub mod application;
pub mod domain;
pub mod ports;

// Re-exports for convenience
pub use application::commands::*;
pub use application::queries::*;
pub use domain::entities::*;
pub use domain::events::*;
pub use domain::value_objects::*;
pub use ports::driven::CachePort;
pub use ports::driven::SingleflightPort;

/// Two-tier cache re-export for convenience
pub mod cache {
    pub use crate::adapters::inmemory::TieredCache;
}

#[cfg(feature = "python")]
pub mod python {
    pub use crate::adapters::python::PythonCache;
}
