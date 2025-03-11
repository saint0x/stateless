//! Server implementation for the stateless caching system

mod shard;
mod storage;
mod network;
mod protocol;

use core::prelude::*;
use std::sync::Arc;

/// Server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Number of shards (defaults to number of CPU cores)
    pub num_shards: Option<usize>,
    /// Maximum memory usage
    pub max_memory: usize,
    /// Data directory for persistence
    pub data_dir: std::path::PathBuf,
    /// Network configuration
    pub network: NetworkConfig,
}

/// Network configuration
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    /// Listen address
    pub listen_addr: std::net::SocketAddr,
    /// TLS configuration
    pub tls: Option<TlsConfig>,
}

/// TLS configuration
#[derive(Debug, Clone)]
pub struct TlsConfig {
    /// Certificate file
    pub cert_file: std::path::PathBuf,
    /// Key file
    pub key_file: std::path::PathBuf,
}

/// Server instance
pub struct Server {
    config: ServerConfig,
    shards: Vec<Arc<shard::Shard>>,
    storage: Arc<storage::Storage>,
    network: Arc<network::Network>,
}

impl Server {
    /// Create a new server instance
    pub async fn new(config: ServerConfig) -> core::Result<Self> {
        let num_shards = config.num_shards.unwrap_or_else(num_cpus::get);
        let mut shards = Vec::with_capacity(num_shards);
        
        // Initialize shards
        for i in 0..num_shards {
            shards.push(Arc::new(shard::Shard::new(i, &config)?));
        }
        
        // Initialize storage
        let storage = Arc::new(storage::Storage::new(&config)?);
        
        // Initialize network
        let network = Arc::new(network::Network::new(&config)?);
        
        Ok(Self {
            config,
            shards,
            storage,
            network,
        })
    }
    
    /// Start the server
    pub async fn run(&self) -> core::Result<()> {
        // Start background tasks
        self.start_background_tasks();
        
        // Start network server
        self.network.run(self.shards.clone()).await
    }
    
    /// Start background maintenance tasks
    fn start_background_tasks(&self) {
        // Shard maintenance
        for shard in &self.shards {
            let shard = shard.clone();
            tokio::spawn(async move {
                shard.run_maintenance().await;
            });
        }
        
        // Storage maintenance
        let storage = self.storage.clone();
        tokio::spawn(async move {
            storage.run_maintenance().await;
        });
    }
}

// Re-exports
pub use shard::Shard;
pub use storage::Storage;
pub use network::Network;
pub use protocol::Protocol; 