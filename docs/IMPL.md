# Implementation Guide: stateless SDK

## Quick Start (Redis-Like Usage)

### Simple Key-Value Operations
```rust
// Dead simple initialization
let cache = stateless::quick_start();

// Redis-like operations
cache.set("user:123", user).await?;
cache.get("user:123").await?;
cache.del("user:123").await?;
cache.expire("user:123", 60).await?;  // 60 seconds TTL

// Basic patterns work as expected
cache.incr("visits").await?;
cache.lpush("queue", item).await?;
cache.sadd("set", member).await?;
```

### Drop-in Redis Replacement
```rust
// For existing Redis codebases
let cache = stateless::redis_compatible()
    .with_safety()  // Adds our guarantees silently
    .build()?;

// Your existing Redis code works unchanged
redis::cmd("SET")
    .arg("key")
    .arg("value")
    .execute(&mut cache);
```

### Common Patterns
```rust
// Function memoization
#[stateless::cache(ttl = "5m")]
async fn expensive_calculation() -> Result<Value> {
    // Result is cached automatically
}

// Rate limiting
let limiter = cache.rate_limiter("api", "100/minute");
if limiter.allow_request()? {
    // Handle request
}

// Session handling
app.use(stateless::session({
    ttl: "24h",
    rolling: true
}));
```

## Strategic Caching (Power Features)

### Cache Manifest
Define your caching strategy when you need more control:

```rust
#[cache_manifest]
struct AppCache {
    #[client_primary]
    user_data: UserDataStrategy,
    
    #[edge_primary]
    content: ContentStrategy,
    
    #[server_primary]
    analytics: AnalyticsStrategy,
}
```

### Progressive Enhancement
Start simple and add power features as needed:

```rust
// 1. Start with basic caching
let cache = Cache::for_strategy::<SimpleStrategy>();
cache.set("user:123", user).await?;

// 2. Add offline support
let cache = cache.with_offline_support();
cache.set("user:123", user)
    .with_offline_fallback()
    .await?;

// 3. Add edge caching
let cache = cache.with_edge_caching();
cache.set("user:123", user)
    .with_offline_fallback()
    .with_edge_sync(ttl = "1h")
    .await?;
```

## Borrow Checker Integration

### Automatic Safety (Even in Simple Mode)
```rust
// Even in simple mode, the borrow checker prevents conflicts
cache.set("user:123", user1).await?;
cache.set("user:123", user2).await?;  // OK: Simple override

// But prevents obvious mistakes
cache.set_client("user:123", user1).await?;
cache.set_server("user:123", user2).await?;  // Error: Conflicting cache locations
```

### Strategic Safety
```rust
#[cache_manifest]
struct AppCache {
    #[client_primary]
    #[owns = "user:*"]
    user_data: UserStrategy,
}

// Compiler enforces strategy
#[cache(follows = "user_data")]
async fn update_user(user: &User) {
    cache.set(&format!("user:{}", user.id), user).await?;
}
```

## Advanced Features

### Smart Defaults
```rust
// Auto-detect optimal strategy
let cache = stateless::auto()
    .with_safety()
    .build()?;

// SDK automatically:
// - Chooses appropriate storage locations
// - Sets reasonable TTLs
// - Configures fallback behavior
// - Maintains borrow checker rules
```

### Transaction Support
```rust
// Redis-style transactions
cache.transaction(|tx| {
    tx.set("key1", "value1")?;
    tx.set("key2", "value2")?;
    Ok(())
})?;

// Strategic transactions
cache.safe_transaction(|tx| {
    // Compiler ensures all keys follow strategy
    tx.set("user:123", user1)?;
    tx.set("user:456", user2)?;
    Ok(())
})?;
```

### Pattern Matching
```rust
// Redis-like pattern operations
cache.del_pattern("user:*").await?;
cache.expire_pattern("session:*", 3600).await?;

// But with strategic safety
cache.del_pattern("user:*")
    .verify_ownership()  // Ensures we own this pattern
    .await?;
```

### Framework Integration
```rust
// Next.js style
export const getStaticProps = stateless::cache({
    strategy: "edge_primary",
    revalidate: 60
});

// Express/Actix style
app.use(stateless::cache_middleware({
    strategy: "client_primary",
    fallback: "edge"
}));
```

## CLI Tools

### Interactive Shell
```bash
# Redis-like CLI
$ stateless-cli
stateless> SET user:123 "John"
OK
stateless> GET user:123
"John"
stateless> STRATEGY user:123
"client_primary (fallback: edge)"
```

### Monitoring
```bash
# Built-in monitoring
$ stateless monitor
Watching cache operations...
SET user:123 - client_primary - 0.5ms
GET user:456 - edge_fallback - 1.2ms
```

## Migration Guide

### From Redis
1. **Direct Replacement**
```rust
// Step 1: Drop-in replacement
let cache = stateless::redis_compatible();

// Step 2: Enable safety features
let cache = stateless::redis_compatible()
    .with_safety()
    .build()?;

// Step 3: Gradually adopt strategies
let cache = cache.into_strategic_cache::<UserStrategy>();
```

### From Basic to Strategic
```rust
// Step 1: Basic usage
let cache = stateless::quick_start();

// Step 2: Add manifest
#[cache_manifest]
struct AppCache {
    #[client_primary]
    user_data: SimpleStrategy,
}

// Step 3: Enhance with features
#[cache_manifest]
struct AppCache {
    #[client_primary]
    #[with_offline_fallback]
    #[with_edge_sync(ttl = "1h")]
    user_data: AdvancedStrategy,
}
```

## Best Practices

1. **Start Simple**
   - Use quick_start() for basic needs
   - Add strategy when patterns emerge
   - Let the SDK guide you to better practices

2. **Safety First**
   - Even in simple mode, get basic safety
   - Graduate to strategic safety when ready
   - Let the borrow checker prevent mistakes

3. **Performance**
   - Simple usage is Redis-fast
   - Strategic usage optimizes automatically
   - Advanced features when you need them

4. **Migration**
   - Start with Redis compatibility
   - Enable safety features gradually
   - Adopt strategies when beneficial

## SDK Extension

### Custom Strategies
```rust
struct CustomStrategy;

impl CacheStrategy for CustomStrategy {
    fn cache_location(&self, key: &str) -> CacheLocation {
        // Custom logic for cache location
    }
    
    fn fallback_strategy(&self) -> FallbackStrategy {
        // Custom fallback logic
    }
}
```

### Custom Cache Locations
```rust
#[derive(CacheLocation)]
enum CustomLocation {
    CloudflareWorker,
    CloudFront,
    Lambda,
} 