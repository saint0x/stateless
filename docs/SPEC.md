# System Specification: stateless SDK

## 1. Core Concepts

### Borrow Checker Model
The stateless SDK introduces a novel borrow checker for caches, inspired by Rust's memory safety model:

```rust
// Core ownership example
#[cache_manifest]
struct AppCache {
    #[owns = "user:*"]
    user_data: UserCache,
    
    #[owns = "product:*"]
    product_data: ProductCache,
    
    #[borrows = "user:*", "product:*"]
    analytics: AnalyticsCache,
}
```

#### Key Properties
1. **Exclusive Ownership**: Each cache pattern is owned by exactly one component
2. **Borrowing Rules**: Multiple readers OR one writer, never both
3. **Lifetime Guarantees**: Cache entries cannot outlive their owners
4. **Cross-Layer Safety**: Prevents conflicts between client/edge/server caches

### Pattern System
Cache patterns define the scope and relationships of cached data:

```rust
// Pattern examples
"user:*"           // All user data
"user:{id}"        // Specific user
"user:{id}:posts"  // User's posts
"product:featured" // Featured products
```

#### Pattern Rules
1. **Prefix Ownership**: Owning "user:*" grants control over all user data
2. **Pattern Specificity**: More specific patterns take precedence
3. **Cross-Pattern Relations**: Explicit dependency declarations required

## 2. Cache Manifest

### Structure
```rust
#[cache_manifest]
struct AppCache {
    #[layer = "client", owns = "local:*"]
    local: LocalCache,
    
    #[layer = "edge", owns = "edge:*"]
    edge: EdgeCache,
    
    #[layer = "server", owns = "global:*"]
    server: ServerCache,
}
```

### Layer Rules
1. **Client Layer**
   - Owns user-specific data
   - Local-first operations
   - Offline support

2. **Edge Layer**
   - Regional data ownership
   - TTL enforcement
   - Proximity routing

3. **Server Layer**
   - Global state ownership
   - Strong consistency
   - Master data source

## 3. Safety Guarantees

### Compile-Time Checks
```rust
// These will fail at compile time:

// Error: Pattern conflict
#[owns = "user:*"]
struct UserCache {}

#[owns = "user:profile"]  // Conflicts with "user:*"
struct ProfileCache {}

// Error: Invalid borrowing
#[borrows = "product:*"]
fn update_product() {     // Cannot modify borrowed data
    cache.set("product:1", data);
}
```

### Runtime Guarantees
1. **Consistency**
   - No conflicting writes
   - Atomic operations
   - Ordered invalidations

2. **Performance**
   - No unnecessary network calls
   - Optimal layer selection
   - Efficient pattern matching

## 4. Cache Operations

### Basic Usage
```rust
// Simple Redis-like operations
cache.get("user:123").await?;
cache.set("user:123", data).await?;
cache.del("user:123").await?;

// With ownership checking
#[cache(owns = "user:*")]
async fn update_user(id: &str, data: UserData) {
    cache.set(&format!("user:{}", id), data).await?;
}
```

### Advanced Features
```rust
// Pattern-based operations
cache.invalidate_pattern("user:123:*").await?;

// Multi-layer operations
cache.set_with_strategy("product:featured", data, Strategy::EdgeFirst).await?;

// Batch operations
cache.atomic_batch(|tx| {
    tx.set("user:123", data1);
    tx.set("user:123:prefs", data2);
}).await?;
```

## 5. Strategy System

### Built-in Strategies
1. **ClientFirst**
   - Prioritize client storage
   - Offline-capable
   - Background sync

2. **EdgeOptimized**
   - Geographic routing
   - Regional consistency
   - TTL-based invalidation

3. **GlobalConsistent**
   - Strong consistency
   - Master-slave replication
   - Global invalidation

### Custom Strategies
```rust
struct CustomStrategy {
    rules: StrategyRules,
    fallback: Box<dyn CacheStrategy>,
}

impl CacheStrategy for CustomStrategy {
    fn determine_location(&self, key: &str) -> CacheLayer;
    fn handle_invalidation(&self, pattern: &str) -> Vec<String>;
}
```

## 6. Error Handling

### Compile-Time Errors
1. **Pattern Conflicts**
   - Overlapping ownership
   - Invalid borrowing
   - Layer violations

2. **Strategy Errors**
   - Invalid compositions
   - Unreachable layers
   - Circular dependencies

### Runtime Errors
1. **Operation Errors**
   - Network failures
   - Capacity limits
   - Timeout issues

2. **Consistency Errors**
   - Version conflicts
   - Stale data
   - Invalid states

## 7. Performance Model

### Latency Targets
- Local cache: < 1ms
- Edge cache: < 10ms
- Server cache: < 50ms

### Throughput Goals
- Client: 10K ops/sec
- Edge: 100K ops/sec
- Server: 1M ops/sec

## 8. Migration Support

### From Redis
```rust
// Automatic Redis command translation
let redis_cache = RedisAdapter::new(cache);
redis_cache.set("key", "value").await?;

// Strategy migration
redis_cache.with_strategy(Strategy::RedisCompatible);
```

### Progressive Enhancement
1. Start with basic Redis-like usage
2. Add ownership annotations gradually
3. Implement advanced strategies
4. Enable cross-layer optimization

## 9. Best Practices

### Pattern Design
1. Use specific patterns when possible
2. Group related data under common prefixes
3. Consider layer boundaries in pattern design

### Strategy Selection
1. Start with built-in strategies
2. Customize for specific use cases
3. Monitor and adjust based on metrics

### Performance Optimization
1. Leverage compile-time validation
2. Use batch operations when possible
3. Implement appropriate fallback strategies