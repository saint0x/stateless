# Core Features & Syntax

## Cache Manifest Attributes
```rust
#[cache_manifest]                    -> Enable cache manifest processing
#[owns = "pattern"]                  -> Declare pattern ownership
#[borrows = "pattern"]               -> Declare pattern borrowing
#[layer = "client|edge|server"]      -> Specify cache layer
#[invalidates = "pattern"]           -> Declare invalidation rules
```

## Pattern Syntax
```rust
"exact:key"                          -> Exact key match
"prefix:*"                           -> Prefix wildcard match
"user:{id}"                          -> Variable substitution
"user:{id}:*"                       -> Combined variable and wildcard
"prefix:[a-z]+"                     -> Regex pattern support
```

## Basic Operations
```rust
cache.get(key)                       -> Simple key retrieval
cache.set(key, value)                -> Simple key-value storage
cache.del(key)                       -> Key deletion
cache.exists(key)                    -> Key existence check
cache.ttl(key)                       -> Get TTL for key
```

## Pattern Operations
```rust
cache.get_pattern("user:*")          -> Get all matching keys
cache.invalidate_pattern("user:*")    -> Invalidate matching keys
cache.watch_pattern("user:*")        -> Watch for changes
cache.list_patterns()                -> List active patterns
```

## Atomic Operations
```rust
cache.atomic_batch(|tx| {})          -> Atomic transaction block
cache.cas(key, old, new)             -> Compare and swap
cache.incr(key)                      -> Atomic increment
cache.decr(key)                      -> Atomic decrement
```

## Strategy Control
```rust
Strategy::ClientFirst               -> Client-prioritized caching
Strategy::EdgeOptimized            -> Edge-optimized caching
Strategy::GlobalConsistent         -> Global consistency mode
Strategy::RedisCompatible          -> Redis compatibility mode
```

## Layer Control
```rust
cache.set_layer(key, Layer::Client)  -> Force client layer
cache.get_from(key, Layer::Edge)     -> Read from specific layer
cache.replicate_to(Layer::Edge)      -> Replicate to layer
cache.pin_to(Layer::Server)          -> Pin to specific layer
```

## Ownership Control
```rust
cache.transfer(from, to)             -> Transfer ownership
cache.borrow_from(owner)             -> Temporary borrowing
cache.release_borrow()               -> Release borrowing
cache.check_owner(pattern)           -> Check pattern owner
```

## Metadata Operations
```rust
cache.get_meta(key)                  -> Get key metadata
cache.set_tags(key, tags)            -> Set key tags
cache.find_by_tag(tag)               -> Find keys by tag
cache.get_stats(pattern)             -> Get pattern stats
```

## Lifecycle Hooks
```rust
cache.on_invalidate(pattern, fn)     -> Invalidation callback
cache.on_update(pattern, fn)         -> Update callback
cache.on_expire(pattern, fn)         -> Expiration callback
cache.on_conflict(pattern, fn)       -> Conflict resolution
```

## Migration Support
```rust
RedisAdapter::new(cache)             -> Redis protocol adapter
cache.import_from(source)            -> Import data
cache.export_to(target)              -> Export data
cache.migrate(source, target)        -> Live migration
```

## Monitoring
```rust
cache.metrics()                      -> Get cache metrics
cache.health_check()                 -> Check cache health
cache.trace_pattern(pattern)         -> Pattern access tracing
cache.debug_ownership(pattern)       -> Debug ownership info
``` 