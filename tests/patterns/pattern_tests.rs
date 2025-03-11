use core::prelude::*;
use stateless::Pattern;

#[tokio::test]
async fn test_exact_patterns() {
    let pattern = Pattern::new("user:123");
    
    assert!(pattern.matches("user:123"));
    assert!(!pattern.matches("user:124"));
    assert!(!pattern.matches("user:123:profile"));
}

#[tokio::test]
async fn test_wildcard_patterns() {
    let pattern = Pattern::new("user:*");
    
    assert!(pattern.matches("user:123"));
    assert!(pattern.matches("user:456"));
    assert!(!pattern.matches("profile:123"));
}

#[tokio::test]
async fn test_variable_patterns() {
    let pattern = Pattern::new("user:{id}:profile");
    
    assert!(pattern.matches("user:123:profile"));
    assert!(pattern.matches("user:456:profile"));
    assert!(!pattern.matches("user:123:settings"));
}

#[tokio::test]
async fn test_regex_patterns() {
    let pattern = Pattern::new("user:[0-9]+:posts");
    
    assert!(pattern.matches("user:123:posts"));
    assert!(!pattern.matches("user:abc:posts"));
}

#[tokio::test]
async fn test_pattern_matching_performance() {
    let mut matcher = PatternMatcher::new();
    
    // Add many patterns
    for i in 0..1000 {
        matcher.add(Pattern::new(&format!("user:{}:*", i)));
    }
    
    // Benchmark matching
    let start = std::time::Instant::now();
    for i in 0..1000 {
        let _ = matcher.matches(&format!("user:{}:profile", i));
    }
    let duration = start.elapsed();
    
    // Should be fast
    assert!(duration.as_micros() < 1000); // Less than 1µs per match
}

#[tokio::test]
async fn test_pattern_operations() {
    let cache = stateless::quick_start();
    
    // Set some test data
    cache.set("user:123:profile", "Alice").await?;
    cache.set("user:123:settings", "dark_mode").await?;
    cache.set("user:456:profile", "Bob").await?;
    
    // Test pattern operations
    let keys = cache.get_pattern("user:123:*").await?;
    assert_eq!(keys.len(), 2);
    
    cache.invalidate_pattern("user:123:*").await?;
    assert!(cache.get("user:123:profile").await?.is_none());
    assert!(cache.get("user:456:profile").await?.is_some());
}

#[tokio::test]
async fn test_pattern_watching() {
    let cache = stateless::quick_start();
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    
    // Watch pattern
    cache.watch_pattern("user:*", move |key, _value| {
        tx.try_send(key.to_string()).ok();
    }).await?;
    
    // Make some changes
    cache.set("user:123", "Alice").await?;
    cache.set("user:456", "Bob").await?;
    cache.set("profile:789", "Charlie").await?; // Shouldn't trigger
    
    // Check notifications
    assert_eq!(rx.try_recv().unwrap(), "user:123");
    assert_eq!(rx.try_recv().unwrap(), "user:456");
    assert!(rx.try_recv().is_err());
} 