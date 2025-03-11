//! stateless: A universal caching layer with compile-time safety guarantees
//! 
//! This crate provides a Redis-like caching system with Rust's borrow checker principles
//! applied to cache consistency and performance optimization.

pub use core::{Cache, CacheEntry, Pattern, Strategy, Layer, Error, Result};
pub use macros::{cache_manifest, cache, CacheStrategy};

#[cfg(feature = "redis-compat")]
pub use redis;

#[cfg(feature = "client")]
pub use client;

#[cfg(feature = "edge")]
pub use edge;

/// Quick start with Redis-like simplicity
pub fn quick_start() -> impl Cache {
    todo!("Implement quick start cache")
}

/// Redis-compatible mode
#[cfg(feature = "redis-compat")]
pub fn redis_compatible() -> impl Cache {
    todo!("Implement Redis compatibility")
}

/// Auto-configured cache with safety features
pub fn auto() -> impl Cache {
    todo!("Implement auto-configured cache")
}

/// Re-exports of common types and traits
pub mod prelude {
    pub use core::prelude::*;
    pub use macros::{cache_manifest, cache, CacheStrategy};
    
    #[cfg(feature = "redis-compat")]
    pub use crate::redis_compatible;
    
    pub use crate::{quick_start, auto};
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_quick_start() {
        let cache = quick_start();
        // TODO: Add basic functionality tests
    }
    
    #[tokio::test]
    async fn test_borrow_checker() {
        // TODO: Add borrow checker validation tests
    }
}
