---
layout: home
title: thegent-cache - Multi-tier Caching Library
titleTemplate: false
---

# thegent-cache

Multi-tier caching library for agent orchestration with Hexagonal Architecture.

## Overview

`thegent-cache` (FacetRs) is a Rust library providing multi-tier caching with LRU and TTL support, designed for agent orchestration workloads.

## Features

- **Multi-tier caching**: LRU + DashMap with TTL
- **Singleflight**: Prevent thundering herd
- **Hexagonal Architecture**: Pluggable adapters
- **Python bindings**: Optional pyo3 support

## Quick Start

```rust
use thegent_cache::{Cache, LruCache, TtlCache};

let cache = LruCache::new(1000);
cache.insert("key", "value");
assert_eq!(cache.get(&"key"), Some(&"value"));
```

## Architecture

```
┌─────────────────────────────────────────┐
│              Application                │
├─────────────────────────────────────────┤
│                 Ports                    │
├─────────────────────────────────────────┤
│               Domain                     │
├─────────────────────────────────────────┤
│             Adapters                     │
│    (LruCache, DashMap, TtlCache)        │
└─────────────────────────────────────────┘
```

## Links

- [Repository](https://github.com/KooshaPari/thegent-cache)
- [Documentation](https://docs.rs/FacetRs)
