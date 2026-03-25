//! # Commands (CQRS)
//!
//! Operations that change state.

use crate::domain::value_objects::{CacheKey, CacheValue, Ttl, CacheTier};

/// Command to set a cache value
#[derive(Debug, Clone)]
pub struct SetCacheCommand {
    pub key: CacheKey,
    pub value: CacheValue,
    pub ttl: Option<Ttl>,
}

/// Command to remove a cache entry
#[derive(Debug, Clone)]
pub struct RemoveCacheCommand {
    pub key: CacheKey,
}

/// Command to clear the cache
#[derive(Debug, Clone)]
pub struct ClearCacheCommand {
    pub tier: Option<CacheTier>,
}

/// Command to set cache configuration
#[derive(Debug, Clone)]
pub struct SetConfigCommand {
    pub l1_max_size: Option<usize>,
    pub l2_max_size: Option<usize>,
    pub default_ttl: Option<Ttl>,
}

/// Command to start a singleflight request
#[derive(Debug, Clone)]
pub struct StartSingleflightCommand {
    pub key: String,
    pub requester_pid: u32,
}

/// Command to complete a singleflight request
#[derive(Debug, Clone)]
pub struct CompleteSingleflightCommand<T> {
    pub key: String,
    pub result: Option<T>,
    pub error: Option<String>,
}
