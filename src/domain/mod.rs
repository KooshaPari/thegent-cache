//! # Domain Layer
//!
//! Contains core business logic with no external dependencies.
//!
//! ## DDD Principles Applied
//!
//! - **Entities**: Objects with identity (CacheEntry, SingleflightRequest)
//! - **Value Objects**: Immutable objects (CacheKey, CacheValue, TTL)
//! - **Domain Events**: Immutable events (CacheHit, CacheMiss, etc.)

pub mod entities;
pub mod events;
pub mod value_objects;
