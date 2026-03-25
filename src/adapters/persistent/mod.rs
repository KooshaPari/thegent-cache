//! # Persistent Adapter
//!
//! File-based caching implementation.

use std::collections::HashMap;
use std::fs;
use std::time::{Duration, Instant};

use crate::domain::value_objects::{CacheKey, CacheValue, CacheTier};
use crate::ports::driven::{CachePort, CacheWritePort, CacheError};

/// File-based cache implementation.
pub struct PersistentCache {
    /// In-memory index
    index: HashMap<CacheKey, (CacheValue, Instant)>,
    /// Base directory
    base_path: String,
    /// Default TTL
    default_ttl: Duration,
}

impl PersistentCache {
    /// Create a new persistent cache.
    pub fn new(base_path: impl Into<String>) -> Self {
        let base_path = base_path.into();
        fs::create_dir_all(&base_path).ok();
        Self {
            index: HashMap::new(),
            base_path,
            default_ttl: Duration::from_secs(3600),
        }
    }

    /// Get the file path for a key.
    fn file_path(&self, key: &CacheKey) -> String {
        format!("{}/{}.cache", self.base_path, key.as_str())
    }

    /// Load from disk.
    pub fn load(&mut self) -> Result<usize, CacheError> {
        let mut count = 0;
        if let Ok(entries) = fs::read_dir(&self.base_path) {
            for entry in entries.flatten() {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    if let Some((key, value)) = content.split_once('\n') {
                        let key = CacheKey::new(key.to_string());
                        let value = CacheValue::new(value.to_string());
                        self.index.insert(key, (value, Instant::now()));
                        count += 1;
                    }
                }
            }
        }
        Ok(count)
    }
}

impl CachePort for PersistentCache {
    fn get(&self, key: &CacheKey) -> Option<CacheValue> {
        let path = self.file_path(key);
        fs::read_to_string(&path)
            .ok()
            .and_then(|content| {
                content.split_once('\n').map(|(_, value)| CacheValue::new(value.to_string()))
            })
    }

    fn get_entry(&self, key: &CacheKey) -> Option<crate::domain::entities::CacheEntry> {
        self.get(key).map(|value| {
            crate::domain::entities::CacheEntry::new(key.clone(), value)
        })
    }
}

impl CacheWritePort for PersistentCache {
    fn set(&mut self, key: CacheKey, value: CacheValue) -> Result<(), CacheError> {
        let path = self.file_path(&key);
        fs::write(&path, format!("{}\n{}", key.as_str(), value.as_str()))
            .map_err(|e| CacheError::IoError(e.to_string()))?;
        self.index.insert(key, (value, Instant::now()));
        Ok(())
    }

    fn set_with_ttl(&mut self, key: CacheKey, value: CacheValue, _ttl: crate::domain::value_objects::Ttl) -> Result<(), CacheError> {
        // For persistent cache, TTL is handled on read
        self.set(key, value)
    }

    fn remove(&mut self, key: &CacheKey) -> Result<(), CacheError> {
        let path = self.file_path(key);
        fs::remove_file(&path).ok();
        self.index.remove(key);
        Ok(())
    }

    fn clear(&mut self, tier: Option<CacheTier>) -> Result<usize, CacheError> {
        let mut count = 0;
        if tier.is_none() || tier == Some(CacheTier::L3) {
            if let Ok(entries) = fs::read_dir(&self.base_path) {
                for entry in entries.flatten() {
                    if entry.path().extension().map(|e| e == "cache").unwrap_or(false) {
                        fs::remove_file(entry.path()).ok();
                        count += 1;
                    }
                }
            }
            self.index.clear();
        }
        Ok(count)
    }
}
