//! # Use Cases
//!
//! Application services that orchestrate domain logic.

use crate::domain::events::CacheEvent;
use crate::domain::value_objects::{CacheKey, CacheStats, CacheValue, Ttl};
use crate::ports::driven::{CachePort, CacheWritePort, EventPort, StatsPort};

/// Use case for getting a value from cache
pub struct GetCacheUseCase<'a, C: CachePort, S: StatsPort> {
    cache: &'a C,
    stats: &'a mut S,
}

impl<'a, C: CachePort, S: StatsPort> GetCacheUseCase<'a, C, S> {
    pub fn new(cache: &'a C, stats: &'a mut S) -> Self {
        Self { cache, stats }
    }

    pub fn execute(&mut self, key: &CacheKey) -> Option<CacheValue> {
        if let Some(value) = self.cache.get(key) {
            self.stats
                .record_hit(crate::domain::value_objects::CacheTier::L1);
            Some(value)
        } else {
            self.stats.record_miss();
            None
        }
    }
}

/// Use case for setting a value in cache
pub struct SetCacheUseCase<'a, C: CacheWritePort, E: EventPort> {
    cache: &'a mut C,
    events: &'a mut E,
}

impl<'a, C: CacheWritePort, E: EventPort> SetCacheUseCase<'a, C, E> {
    pub fn new(cache: &'a mut C, events: &'a mut E) -> Self {
        Self { cache, events }
    }

    pub fn execute(
        &mut self,
        key: CacheKey,
        value: CacheValue,
        ttl: Option<Ttl>,
    ) -> Result<(), String> {
        let result = if let Some(ttl) = ttl {
            self.cache.set_with_ttl(key.clone(), value.clone(), ttl)
        } else {
            self.cache.set(key.clone(), value.clone())
        };

        if result.is_ok() {
            self.events.publish(CacheEvent::CacheEntryCreated {
                key: key.to_string(),
                tier: crate::domain::value_objects::CacheTier::L1,
                ttl_secs: ttl.unwrap_or_default().as_secs(),
                timestamp: std::time::SystemTime::now(),
            })?;
        }

        result.map_err(|e| e.to_string())
    }
}

/// Use case for removing a value from cache
pub struct RemoveCacheUseCase<'a, C: CacheWritePort> {
    cache: &'a mut C,
}

impl<'a, C: CacheWritePort> RemoveCacheUseCase<'a, C> {
    pub fn new(cache: &'a mut C) -> Self {
        Self { cache }
    }

    pub fn execute(&mut self, key: &CacheKey) -> Result<(), String> {
        self.cache.remove(key).map_err(|e| e.to_string())
    }
}

/// Use case for getting cache statistics
pub struct GetStatsUseCase<'a, S: StatsPort> {
    stats: &'a S,
}

impl<'a, S: StatsPort> GetStatsUseCase<'a, S> {
    pub fn new(stats: &'a S) -> Self {
        Self { stats }
    }

    pub fn execute(&self) -> CacheStats {
        self.stats.get_stats()
    }
}
