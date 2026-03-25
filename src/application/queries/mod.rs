//! # Queries (CQRS)
//!
//! Operations that read state without side effects.

use crate::domain::value_objects::CacheKey;

/// Query to get a cache value
#[derive(Debug, Clone)]
pub struct GetCacheQuery {
    pub key: CacheKey,
}

/// Query to check if key exists
#[derive(Debug, Clone)]
pub struct ExistsCacheQuery {
    pub key: CacheKey,
}

/// Query to get cache statistics
#[derive(Debug, Clone)]
pub struct GetStatsQuery {}

/// Query to list all keys
#[derive(Debug, Clone)]
pub struct ListKeysQuery {
    pub tier: Option<crate::domain::value_objects::CacheTier>,
    pub limit: Option<usize>,
}

/// Query to get cache size
#[derive(Debug, Clone)]
pub struct GetSizeQuery {}

/// Query to get singleflight request status
#[derive(Debug, Clone)]
pub struct GetSingleflightQuery {
    pub key: String,
}
