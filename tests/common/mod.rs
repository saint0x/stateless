use std::sync::Once;
use tokio::runtime::Runtime;

static INIT: Once = Once::new();

/// Initialize test environment
pub fn init() {
    INIT.call_once(|| {
        // Setup logging
        tracing_subscriber::fmt()
            .with_env_filter("debug")
            .try_init()
            .ok();
    });
}

/// Create test runtime
pub fn runtime() -> Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
}

/// Test cache configuration
#[derive(Debug, Clone)]
pub struct TestConfig {
    pub memory_limit: usize,
    pub persistence: bool,
    pub layer: TestLayer,
}

#[derive(Debug, Clone, Copy)]
pub enum TestLayer {
    Client,
    Edge,
    Server,
}

/// Create test data directory
pub fn test_dir() -> tempfile::TempDir {
    tempfile::tempdir().unwrap()
}

/// Generate test data
pub mod data {
    use rand::{thread_rng, Rng};
    
    pub fn random_string(len: usize) -> String {
        thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }
    
    pub fn random_bytes(len: usize) -> Vec<u8> {
        (0..len).map(|_| thread_rng().gen::<u8>()).collect()
    }
} 