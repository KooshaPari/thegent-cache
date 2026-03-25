//! # thegent-cache CLI
//!
//! Command-line interface for thegent-cache.

use clap::{Parser, Subcommand};
use thegent_cache::adapters::inmemory::TieredCache;
use thegent_cache::domain::value_objects::{CacheKey, CacheValue, Ttl};
use thegent_cache::ports::driven::{CachePort, CacheWritePort, StatsPort};

/// thegent-cache CLI
#[derive(Parser, Debug)]
#[command(name = "thegent-cache")]
#[command(about = "Multi-tier caching CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Set a value in cache
    Set {
        /// Cache key
        key: String,
        /// Cache value
        value: String,
        /// TTL in seconds (optional)
        #[arg(short, long)]
        ttl: Option<u64>,
    },
    /// Get a value from cache
    Get {
        /// Cache key
        key: String,
    },
    /// Remove a value from cache
    Rm {
        /// Cache key
        key: String,
    },
    /// List all keys
    List {
        /// Maximum number of keys to show
        #[arg(short, long, default_value = "100")]
        limit: usize,
    },
    /// Clear cache
    Clear {
        /// Tier to clear (optional, clears all if not specified)
        #[arg(short, long)]
        tier: Option<String>,
    },
    /// Show statistics
    Stats,
}

fn main() {
    let cli = Cli::parse();
    let mut cache = TieredCache::default();

    match cli.command {
        Commands::Set { key, value, ttl } => {
            let key_clone = key.clone();
            let value_clone = value.clone();
            let ttl = ttl.map(Ttl::from_secs);
            if let Some(t) = ttl {
                cache.set_with_ttl(key.into(), value.into(), t).unwrap();
            } else {
                cache.set(key.into(), value.into()).unwrap();
            }
            println!("Set {} = {}", key_clone, value_clone);
        }
        Commands::Get { key } => {
            let key_clone = key.clone();
            match cache.get(&key.into()) {
                Some(value) => println!("{}", value),
                None => {
                    eprintln!("Key not found: {}", key_clone);
                    std::process::exit(1);
                }
            }
        }
        Commands::Rm { key } => {
            let key_clone = key.clone();
            cache.remove(&key.into()).unwrap();
            println!("Removed {}", key_clone);
        }
        Commands::List { limit: _ } => {
            println!("Cache contents (L1 + L2):");
            // Note: In-memory adapter doesn't expose iteration directly
            println!("  Use 'stats' to see cache statistics");
        }
        Commands::Clear { tier } => {
            let tier = tier.and_then(|t| match t.as_str() {
                "L1" | "l1" => Some(thegent_cache::domain::value_objects::CacheTier::L1),
                "L2" | "l2" => Some(thegent_cache::domain::value_objects::CacheTier::L2),
                _ => None,
            });
            let count = cache.clear(tier).unwrap();
            println!("Cleared {} entries", count);
        }
        Commands::Stats => {
            let stats = cache.get_stats();
            println!("Cache Statistics:");
            println!("  Hits:    {}", stats.hits);
            println!("  Misses:  {}", stats.misses);
            println!("  Size:    {}", stats.size);
            println!("  Hit Rate: {:.2}%", stats.hit_rate() * 100.0);
            println!("  L1 Hits: {}", stats.tier_hits[0]);
            println!("  L2 Hits: {}", stats.tier_hits[1]);
        }
    }
}
