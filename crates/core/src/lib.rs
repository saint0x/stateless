//! Core types and traits for the stateless caching system

mod pattern;
mod ownership;
mod strategy;
mod layer;
mod error;
mod cache;

pub use pattern::{Pattern, PatternMatcher};
pub use ownership::{Ownership, OwnershipGraph};
pub use strategy::{Strategy, CacheStrategy};
pub use layer::{Layer, LayerCoordinator};
pub use error::{Error, Result};
pub use cache::{Cache, CacheEntry};

/// Re-exports of common traits
pub mod prelude {
    pub use super::{Pattern, PatternMatcher};
    pub use super::{Ownership, OwnershipGraph};
    pub use super::{Strategy, CacheStrategy};
    pub use super::{Layer, LayerCoordinator};
    pub use super::{Cache, CacheEntry};
    pub use super::{Error, Result};
}

// Core pattern system
pub mod pattern {
    use std::fmt;
    use async_trait::async_trait;
    
    /// A pattern that can match cache keys
    #[async_trait]
    pub trait Pattern: fmt::Debug + Send + Sync + 'static {
        /// Check if this pattern matches a key
        fn matches(&self, key: &str) -> bool;
        
        /// Get all keys matching this pattern
        async fn matching_keys(&self) -> crate::Result<Vec<String>>;
    }
    
    /// Engine for efficient pattern matching
    pub struct PatternMatcher {
        // Implementation will use a trie-based system
    }
}

// Ownership tracking
pub mod ownership {
    use std::sync::Arc;
    use dashmap::DashMap;
    
    /// Represents ownership of cache patterns
    pub struct Ownership {
        pattern: String,
        layer: crate::Layer,
        constraints: Vec<Constraint>,
    }
    
    /// Graph of ownership relationships
    pub struct OwnershipGraph {
        nodes: DashMap<String, Arc<Ownership>>,
        edges: DashMap<String, Vec<DependencyEdge>>,
    }
}

// Strategy system
pub mod strategy {
    use async_trait::async_trait;
    
    /// Core strategy trait
    #[async_trait]
    pub trait CacheStrategy: Send + Sync + 'static {
        /// Determine cache location for a key
        async fn determine_location(&self, key: &str) -> crate::Result<crate::Layer>;
        
        /// Handle invalidation for a pattern
        async fn handle_invalidation(&self, pattern: &str) -> crate::Result<Vec<String>>;
    }
}

// Layer coordination
pub mod layer {
    use async_trait::async_trait;
    
    /// Available cache layers
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Layer {
        Client,
        Edge,
        Server,
    }
    
    /// Coordinates operations across layers
    pub struct LayerCoordinator {
        layers: Vec<Box<dyn CacheLayer>>,
        ownership_graph: Arc<crate::OwnershipGraph>,
    }
    
    /// Interface for a cache layer
    #[async_trait]
    pub trait CacheLayer: Send + Sync + 'static {
        async fn get(&self, key: &str) -> crate::Result<Option<Vec<u8>>>;
        async fn set(&self, key: &str, value: Vec<u8>) -> crate::Result<()>;
        async fn delete(&self, key: &str) -> crate::Result<()>;
    }
}

// Error handling
pub mod error {
    use thiserror::Error;
    
    #[derive(Error, Debug)]
    pub enum Error {
        #[error("Pattern conflict: {0}")]
        PatternConflict(String),
        
        #[error("Invalid borrowing: {0}")]
        InvalidBorrowing(String),
        
        #[error("Layer violation: {0}")]
        LayerViolation(String),
        
        #[error("Strategy error: {0}")]
        StrategyError(String),
        
        #[error(transparent)]
        Other(#[from] Box<dyn std::error::Error + Send + Sync>),
    }
    
    pub type Result<T> = std::result::Result<T, Error>;
}

// Core cache interface
pub mod cache {
    use async_trait::async_trait;
    use bytes::Bytes;
    use std::time::Duration;
    
    /// Main cache interface
    #[async_trait]
    pub trait Cache: Send + Sync + 'static {
        async fn get(&self, key: &str) -> crate::Result<Option<CacheEntry>>;
        async fn set(&self, key: &str, value: CacheEntry) -> crate::Result<()>;
        async fn delete(&self, key: &str) -> crate::Result<()>;
        async fn exists(&self, key: &str) -> crate::Result<bool>;
        async fn expire(&self, key: &str, ttl: Duration) -> crate::Result<bool>;
    }
    
    /// A cache entry with metadata
    #[derive(Clone, Debug)]
    pub struct CacheEntry {
        pub value: Bytes,
        pub ttl: Option<Duration>,
        pub metadata: HashMap<String, String>,
    }
} 