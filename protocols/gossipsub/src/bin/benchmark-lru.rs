use libp2p_gossipsub::time_cache::benchmark_helpers::find_max_inserts_per_sec;
use lru_time_cache::LruCache;
use std::error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    find_max_inserts_per_sec(
        |ttl| LruCache::with_expiry_duration(ttl),
        0.1,
        Duration::from_secs(1),
        Duration::from_secs(10),
        10000,
    );
    Ok(())
}
