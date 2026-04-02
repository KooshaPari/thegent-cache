#![feature(test)]
extern crate test;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;
use thegent_cache::{
    domain::entities::{CacheEntry, SingleflightRequest},
    domain::value_objects::{CacheConfig, CacheKey, CacheStats, CacheTier, CacheValue, Ttl},
};

fn bench_cache_key_new(c: &mut Criterion) {
    c.bench_function("cache_key_new", |b| {
        b.iter(|| CacheKey::new(black_box("benchmark_key")));
    });
}

fn bench_cache_value_new(c: &mut Criterion) {
    c.bench_function("cache_value_new", |b| {
        b.iter(|| CacheValue::new(black_box("benchmark_value")));
    });
}

fn bench_ttl_from_secs(c: &mut Criterion) {
    c.bench_function("ttl_from_secs", |b| {
        b.iter(|| Ttl::from_secs(black_box(3600)));
    });
}

fn bench_ttl_from_duration(c: &mut Criterion) {
    c.bench_function("ttl_from_duration", |b| {
        b.iter(|| Ttl::from_duration(black_box(Duration::from_secs(60))));
    });
}

fn bench_cache_entry_new(c: &mut Criterion) {
    let key = CacheKey::new("test_key");
    let value = CacheValue::new("test_value");
    c.bench_function("cache_entry_new", |b| {
        b.iter(|| CacheEntry::new(black_box(key.clone()), black_box(value.clone())));
    });
}

fn bench_cache_entry_with_ttl(c: &mut Criterion) {
    let key = CacheKey::new("test_key");
    let value = CacheValue::new("test_value");
    let ttl = Ttl::from_secs(3600);
    c.bench_function("cache_entry_with_ttl", |b| {
        b.iter(|| {
            CacheEntry::with_ttl(black_box(key.clone()), black_box(value.clone()), black_box(ttl))
        });
    });
}

fn bench_cache_entry_is_expired(c: &mut Criterion) {
    let key = CacheKey::new("test_key");
    let value = CacheValue::new("test_value");
    let entry = CacheEntry::new(key, value);
    c.bench_function("cache_entry_is_expired", |b| {
        b.iter(|| entry.is_expired());
    });
}

fn bench_cache_entry_record_hit(c: &mut Criterion) {
    let key = CacheKey::new("test_key");
    let value = CacheValue::new("test_value");
    let mut entry = CacheEntry::new(key, value);
    c.bench_function("cache_entry_record_hit", |b| {
        b.iter(|| entry.record_hit());
    });
}

fn bench_cache_stats_new(c: &mut Criterion) {
    c.bench_function("cache_stats_new", |b| {
        b.iter(|| CacheStats::new());
    });
}

fn bench_cache_stats_record_hit(c: &mut Criterion) {
    let mut stats = CacheStats::new();
    c.bench_function("cache_stats_record_hit", |b| {
        b.iter(|| stats.record_hit(black_box(CacheTier::L1)));
    });
}

fn bench_cache_stats_hit_rate(c: &mut Criterion) {
    let mut stats = CacheStats::new();
    stats.record_hit(CacheTier::L1);
    stats.record_hit(CacheTier::L2);
    stats.record_miss();
    c.bench_function("cache_stats_hit_rate", |b| {
        b.iter(|| stats.hit_rate());
    });
}

fn bench_cache_config_new(c: &mut Criterion) {
    let ttl = Ttl::from_secs(3600);
    c.bench_function("cache_config_new", |b| {
        b.iter(|| CacheConfig::new(black_box(1000), black_box(10000), black_box(ttl)));
    });
}

fn bench_singleflight_request_new(c: &mut Criterion) {
    c.bench_function("singleflight_request_new", |b| {
        b.iter(|| SingleflightRequest::<String>::new(black_box("test_key")));
    });
}

fn bench_singleflight_request_add_waiter(c: &mut Criterion) {
    let mut req: SingleflightRequest<String> = SingleflightRequest::new("test_key");
    c.bench_function("singleflight_request_add_waiter", |b| {
        b.iter(|| req.add_waiter());
    });
}

fn bench_singleflight_request_set_result(c: &mut Criterion) {
    let mut req: SingleflightRequest<String> = SingleflightRequest::new("test_key");
    c.bench_function("singleflight_request_set_result", |b| {
        b.iter(|| req.set_result(black_box("result".to_string())));
    });
}

criterion_group!(
    benches,
    bench_cache_key_new,
    bench_cache_value_new,
    bench_ttl_from_secs,
    bench_ttl_from_duration,
    bench_cache_entry_new,
    bench_cache_entry_with_ttl,
    bench_cache_entry_is_expired,
    bench_cache_entry_record_hit,
    bench_cache_stats_new,
    bench_cache_stats_record_hit,
    bench_cache_stats_hit_rate,
    bench_cache_config_new,
    bench_singleflight_request_new,
    bench_singleflight_request_add_waiter,
    bench_singleflight_request_set_result
);
criterion_main!(benches);
