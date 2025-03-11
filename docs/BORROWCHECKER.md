# The stateless Borrow Checker

## Core Concept
Just as Rust's borrow checker prevents memory safety issues at compile-time, the stateless borrow checker prevents cache consistency and performance issues across distributed systems.

## Common Cache Disasters We Prevent

### 1. Cache Thrashing
**Problem:** Multiple layers fighting over the same data
```rust
// ❌ Without borrow checker
// Frontend and backend both trying to own user data
frontend.set("user:123", userData);  // Writes to client cache
backend.set("user:123", userData);   // Overwrites in server cache
// Result: Data ping-pongs between layers, wasting bandwidth

// ✅ With borrow checker
#[cache_manifest]
struct AppCache {
    #[client_primary]  // Explicitly declare ownership
    #[owns = "user:*"]
    user_data: UserStrategy,
}

// Now this fails at compile time:
#[cache(follows = "user_data")]
fn backend_cache() {
    backend.set("user:123", data);  // Error: Cannot own user:* in backend
}
```

### 2. Redundant Caching
**Problem:** Same data cached everywhere
```rust
// ❌ Without borrow checker
// Large product catalog cached at every layer
client.set("products:*", catalog);   // 50MB in browser
edge.set("products:*", catalog);     // 50MB in CDN
server.set("products:*", catalog);   // 50MB in server
// Result: 150MB total for same data

// ✅ With borrow checker
#[cache_manifest]
struct AppCache {
    #[edge_primary]
    #[owns = "products:*"]
    #[max_size = "50MB"]
    product_cache: ProductStrategy,
}

// Compiler enforces single source of truth
#[cache(borrows = "products:*")]
fn read_products() {
    // Can only read from edge, preventing redundancy
}
```

### 3. Cache Invalidation Hell
**Problem:** Inconsistent updates across layers
```rust
// ❌ Without borrow checker
// Update user data
server.set("user:123", new_data);
// Forget to invalidate edge cache
// Forget to notify client cache
// Result: Different layers see different data

// ✅ With borrow checker
#[cache(owns = "user:*", invalidates = ["profile:*", "settings:*"])]
async fn update_user(user: User) {
    // Compiler ensures all related caches are invalidated
    cache.set(&format!("user:{}", user.id), user).await?;
    // Automatic invalidation of dependent caches
}
```

### 4. Network Bandwidth Waste
**Problem:** Inefficient data transfer between layers
```rust
// ❌ Without borrow checker
// Large data transferred unnecessarily
edge.get("video:*");  // Downloads 1GB video
client.get("video:*"); // Downloads same 1GB video
// Result: 2GB transfer for 1GB data

// ✅ With borrow checker
#[cache_manifest]
struct AppCache {
    #[edge_primary]
    #[streaming = true]
    #[chunk_size = "1MB"]
    video_cache: VideoStrategy,
}

// Compiler enforces streaming access
#[cache(follows = "video_cache")]
async fn play_video() {
    // Automatically streams in chunks
    let stream = cache.stream("video:123").await?;
}
```

### 5. Cold Start Amplification
**Problem:** Cache misses cascade across layers
```rust
// ❌ Without borrow checker
// Cache miss causes waterfall of requests
client.get("data") // Miss
  -> edge.get("data") // Miss
    -> server.get("data") // Miss
      -> database() // Finally hit
// Result: Slow response, excessive load

// ✅ With borrow checker
#[cache_manifest]
struct AppCache {
    #[warm_strategy]
    #[prefetch = "popular:*"]
    #[backfill = true]
    data_cache: WarmStartStrategy,
}

// Compiler ensures warm starts
#[cache(follows = "data_cache")]
async fn init_cache() {
    // Automatically handles warming
}
```

### 6. Memory Leaks
**Problem:** Forgotten cache entries consume memory
```rust
// ❌ Without borrow checker
// Temporary cache entries never cleaned up
cache.set_temp("req:123", data);
// Developer forgets to delete
// Result: Memory leak

// ✅ With borrow checker
#[cache(lifetime = "request")]
async fn handle_request() {
    // Compiler ensures cleanup at end of request
    cache.set_scoped("req:123", data).await?;
} // Auto-cleanup here
```

### 7. Cross-Layer Race Conditions
**Problem:** Timing issues between cache layers
```rust
// ❌ Without borrow checker
// Race between client and server
if client.get("user:123").is_none() {
    // Another client could write here
    server.set("user:123", data);
}
// Result: Inconsistent state

// ✅ With borrow checker
#[cache(exclusive = "user:*")]
async fn update_user() {
    // Compiler ensures exclusive access across all layers
    cache.atomic_update("user:123", |data| {
        // Safe update
    }).await?;
}
```

## Borrow Checker Rules

### 1. Ownership Rules
```rust
// Single owner for each cache pattern
#[cache_manifest]
struct AppCache {
    #[client_primary]
    #[owns = "user:*"]
    user_cache: UserStrategy,

    #[edge_primary]
    #[owns = "content:*"]
    content_cache: ContentStrategy,
}
```

### 2. Borrowing Rules
```rust
// Read-only access to owned data
#[cache(borrows = "user:*")]
async fn read_user() {
    // Can read but not write
}

// Mutable access requires ownership
#[cache(owns = "user:*")]
async fn write_user() {
    // Can write
}
```

### 3. Lifetime Rules
```rust
// Cache entries tied to scope
#[cache(lifetime = "request")]
async fn handle_request() {
    cache.scoped_set("temp:123", data).await?;
} // Auto-cleanup

// Explicit cleanup required for longer lifetimes
#[cache(lifetime = "session")]
async fn handle_session() {
    cache.set("session:123", data)
        .with_cleanup(cleanup_fn)
        .await?;
}
```

### 4. Strategy Rules
```rust
// Strategies must be compatible
#[cache(strategy = "offline_first")]
async fn offline_capable() {
    // Must include client storage
}

#[cache(strategy = "real_time")]
async fn real_time() {
    // Cannot be offline_first
}
```

## Best Practices

### 1. Layer Ownership
- Assign each cache pattern to most appropriate layer
- Consider access patterns and data characteristics
- Use borrowing for read-heavy operations

### 2. Invalidation Strategy
- Explicitly declare dependencies
- Use automatic invalidation where possible
- Consider cascade effects

### 3. Performance Optimization
- Let borrow checker guide optimal data placement
- Use appropriate lifetime scopes
- Leverage compile-time checks for efficiency

### 4. Error Prevention
- Trust the borrow checker warnings
- Design strategies around data flow
- Use type system to enforce rules 