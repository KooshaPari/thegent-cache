//! Integration tests for thegent-cache
//!
//! These tests verify the integration between cache tiers.

use thegent_cache::domain::entities::CacheEntry;
use thegent_cache::domain::value_objects::{CacheKey, CacheValue};

#[test]
fn test_cache_entry_equality() {
    let entry1 = CacheEntry::new(CacheKey::new("key1"), CacheValue::new("value1"));
    let entry2 = CacheEntry::new(CacheKey::new("key1"), CacheValue::new("value1"));

    assert_eq!(entry1.key.as_str(), entry2.key.as_str());
    assert_eq!(entry1.value.as_str(), entry2.value.as_str());
}

#[test]
fn test_cache_entry_new() {
    let entry = CacheEntry::new(CacheKey::new("test_key"), CacheValue::new("test_value"));

    assert_eq!(entry.key.as_str(), "test_key");
    assert_eq!(entry.value.as_str(), "test_value");
}
