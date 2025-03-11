# Technical Specification: stateless SDK

## 1. Core Architecture

### a. Threading & Concurrency Model
**Thread-per-Core with Sharding:**
- Divide cache into shards (1 shard per CPU core) using consistent hashing
- Each shard owns its data structures, eliminating global locks
- Use `std::sync::Arc<Shard>` for thread-safe shard references

**Non-Blocking I/O:**
- Leverage tokio for async runtime
- Epoll/kqueue for OS-level event polling

**Efficient Hash Tables:**
- Lock-free reads with DashMap-like structures
- Write contention minimized via shard partitioning

### b. Memory Management
**Two-Tier Storage:**
- Hot Tier: In-memory concurrent hash maps with LRU eviction
- Warm Tier: Memory-mapped files for disk persistence
- Simple promotion/demotion based on access patterns

**Memory Efficiency:**
- Pre-allocated memory blocks for common object sizes
- Integration with jemalloc for thread-local heaps

### c. Borrow Checker Integration
**Cache Ownership Rules:**
```rust
// Example: Cache entry ownership and borrowing
#[cache(owns = "user:*")]
async fn update_user_profile(user_id: &str) -> Result<()> {
    // Exclusive access to user:* cache entries
    cache.set(&format!("user:{}", user_id), new_data).await?;
    Ok(())
}

#[cache(borrows = "user:*")]
async fn read_user_profile(user_id: &str) -> Result<User> {
    // Shared access to user:* cache entries
    cache.get(&format!("user:{}", user_id)).await
}
```

**Invalidation Tracking:**
- Compile-time dependency validation
- Automatic invalidation graph generation
- Dead entry detection and cleanup

## 2. Server-Side Features

### a. Protocol Support
**Core Protocols:**
- HTTP/2: RESTful API with JSON payloads
- gRPC: For strongly-typed clients
- Redis RESP: Basic compatibility mode

### b. Persistence
**Simple Durability:**
- Write-Ahead Log (WAL) for crash recovery
- Periodic snapshots to disk
- Configurable fsync intervals

## 3. Client-Side SDK Design

### a. Client Architecture
**Type-Safe API:**
```typescript
// TypeScript SDK Example
interface CacheConfig {
  maxSize: number;
  ttl: number;
}

class Cache<T> {
  async get(key: string): Promise<T | null>;
  async set(key: string, value: T, ttl?: number): Promise<void>;
  async delete(key: string): Promise<void>;
}
```

**Local Storage:**
- Browser: IndexedDB backend
- Mobile: SQLite for persistence
- Simple LRU eviction

### b. Edge Integration
**Basic CDN Support:**
- Standard cache-control headers
- TTL-based invalidation
- Regional routing via DNS

## 4. Implementation Examples

### a. Rust Core (Shard Implementation)
```rust
pub struct Shard {
    map: DashMap<String, Vec<u8>>,
    eviction: LruCache<String>,
}

impl Shard {
    pub async fn get(&self, key: &str) -> Option<Vec<u8>> {
        if let Some(entry) = self.map.get(key) {
            self.eviction.touch(key);
            Some(entry.value().clone())
        } else {
            None
        }
    }
}
```

### b. TypeScript SDK
```typescript
class StatelessCache {
    constructor(config: CacheConfig) {
        this.maxSize = config.maxSize;
        this.ttl = config.ttl;
    }

    async get<T>(key: string): Promise<T | null> {
        // Implementation with type safety
        const value = await this.storage.get(key);
        return value ? (JSON.parse(value) as T) : null;
    }
}
```

## 5. Performance Targets

### a. Latency Goals
- p99 < 1ms for cache hits
- p99 < 10ms for cache misses with disk fallback

### b. Throughput
- 1M+ QPS per node
- Linear scaling with core count

## 6. Security & Monitoring

### a. Basic Security
- RBAC for cache operations
- TLS for all network communication
- Audit logging for sensitive operations

### b. Observability
- Prometheus metrics export
- Basic dashboard for cache stats
- Health check endpoints

## 7. Development Roadmap
See ROADMAP.md for detailed implementation phases and timeline.

## 1. Borrow Checker Core

### a. Ownership Graph
```rust
/// Core pattern matching for cache keys
pub struct CachePattern {
    pattern: String,     // e.g., "user:*"
    ownership: Ownership,
    layer: CacheLayer,
    constraints: Vec<Constraint>,
}

/// Graph representation of cache dependencies
pub struct OwnershipGraph {
    nodes: HashMap<String, CachePattern>,
    edges: Vec<DependencyEdge>,
    layer_rules: LayerRules,
}

/// Represents dependencies between patterns
pub struct DependencyEdge {
    from: String,      // Source pattern
    to: String,        // Target pattern
    edge_type: EdgeType,
    constraints: Vec<Constraint>,
}

/// Types of pattern relationships
pub enum EdgeType {
    Owns,             // Full ownership
    Borrows,          // Read-only access
    Invalidates,      // Invalidation relationship
    Derives,          // Derived data relationship
}

impl OwnershipGraph {
    /// Validate cache access at compile time
    pub fn validate_access(&self, pattern: &str, mode: AccessMode) -> Result<()> {
        let pattern_node = self.resolve_pattern(pattern)?;
        
        // Check layer constraints
        self.validate_layer_access(&pattern_node, mode)?;
        
        // Check ownership rules
        self.validate_ownership(&pattern_node, mode)?;
        
        // Check invalidation rules
        self.compute_invalidation_set(&pattern_node)?;
        
        Ok(())
    }

    /// Resolve pattern matches including wildcards
    fn resolve_pattern(&self, pattern: &str) -> Result<Vec<&CachePattern>> {
        // Pattern matching logic including glob support
        // Returns all matching patterns
    }

    /// Validate access based on cache layer
    fn validate_layer_access(&self, pattern: &CachePattern, mode: AccessMode) -> Result<()> {
        match (pattern.layer, mode) {
            (CacheLayer::Client, AccessMode::Write) => {
                // Ensure client can write to this pattern
            }
            (CacheLayer::Edge, _) => {
                // Validate edge layer constraints
            }
            // ... other layer validations
        }
    }
}
```

### b. Pattern Matching Engine
```rust
/// Pattern matching engine
pub struct PatternMatcher {
    patterns: Vec<CachePattern>,
    trie: PatternTrie,
}

impl PatternMatcher {
    /// Fast pattern matching using trie structure
    pub fn matches(&self, key: &str) -> Vec<&CachePattern> {
        self.trie.find_matches(key)
    }

    /// Compile patterns into efficient trie
    fn build_trie(&mut self) {
        for pattern in &self.patterns {
            self.trie.insert(&pattern.pattern, pattern);
        }
    }
}

/// Efficient trie structure for pattern matching
struct PatternTrie {
    root: TrieNode,
    wildcard_optimization: WildcardIndex,
}

impl PatternTrie {
    /// Find all matching patterns for a key
    fn find_matches(&self, key: &str) -> Vec<&CachePattern> {
        // Efficient pattern matching algorithm
        // Uses wildcard optimization for *
    }
}
```

### c. Compile-Time Validation
```rust
/// Procedural macro implementation
#[proc_macro_attribute]
pub fn cache_manifest(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse cache manifest attributes
    let manifest = parse_macro_input!(item as CacheManifest);
    
    // Build ownership graph
    let graph = build_ownership_graph(&manifest);
    
    // Validate patterns and relationships
    graph.validate()?;
    
    // Generate implementation
    quote! {
        // Generated code for cache manifest
    }
}

/// Cache attribute implementation
#[proc_macro_attribute]
pub fn cache(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse cache attributes
    let attrs = parse_macro_input!(attr as CacheAttributes);
    
    // Validate against manifest
    validate_cache_usage(&attrs)?;
    
    // Generate implementation
    quote! {
        // Generated code for cache usage
    }
}
```

## 2. Layer Coordination

### a. Layer-Aware Routing
```rust
/// Cache layer definition
pub enum CacheLayer {
    Client {
        constraints: ClientConstraints,
        storage: ClientStorage,
    },
    Edge {
        region: String,
        ttl: Duration,
        strategy: EdgeStrategy,
    },
    Server {
        role: ServerRole,
        capacity: usize,
    },
}

/// Layer coordination
pub struct LayerCoordinator {
    layers: Vec<CacheLayer>,
    routing_rules: RoutingRules,
    ownership_graph: Arc<OwnershipGraph>,
}

impl LayerCoordinator {
    /// Route cache operation to appropriate layer
    pub async fn route_operation(&self, op: CacheOp) -> Result<()> {
        // Validate operation against ownership graph
        self.ownership_graph.validate_access(&op.pattern, op.mode)?;
        
        // Determine target layer
        let layer = self.determine_layer(&op)?;
        
        // Execute operation
        self.execute_on_layer(layer, op).await
    }
}
```

### b. Cross-Layer Invalidation
```rust
/// Invalidation tracking
pub struct InvalidationTracker {
    dependencies: HashMap<String, Vec<String>>,
    propagation_rules: PropagationRules,
}

impl InvalidationTracker {
    /// Compute invalidation set for an operation
    pub fn compute_invalidation_set(&self, op: &CacheOp) -> Vec<InvalidationTask> {
        // Determine what needs to be invalidated
        let patterns = self.find_dependent_patterns(&op.pattern);
        
        // Create invalidation tasks
        patterns.into_iter()
            .map(|p| InvalidationTask::new(p))
            .collect()
    }
}
```

## 3. Strategy Implementation

### a. Basic Strategies
```rust
/// Core strategy trait
pub trait CacheStrategy {
    fn validate_pattern(&self, pattern: &str) -> Result<()>;
    fn determine_location(&self, key: &str) -> CacheLayer;
    fn handle_invalidation(&self, pattern: &str) -> Vec<String>;
}

/// Client-primary strategy
pub struct ClientPrimaryStrategy {
    ownership_rules: OwnershipRules,
    fallback_config: FallbackConfig,
}

impl CacheStrategy for ClientPrimaryStrategy {
    fn determine_location(&self, key: &str) -> CacheLayer {
        // Implement client-primary logic
    }
}
```

### b. Strategy Composition
```rust
/// Composed strategy
pub struct ComposedStrategy {
    strategies: Vec<Box<dyn CacheStrategy>>,
    composition_rules: CompositionRules,
}

impl CacheStrategy for ComposedStrategy {
    fn determine_location(&self, key: &str) -> CacheLayer {
        // Apply composition rules
        self.composition_rules.apply(key, &self.strategies)
    }
}
```

## 4. Redis Compatibility

### a. Command Implementation
```rust
/// Redis command adapter
pub struct RedisAdapter {
    inner: StatelessCache,
    strategy: Box<dyn CacheStrategy>,
}

impl RedisAdapter {
    /// Handle Redis command
    pub async fn handle_command(&self, cmd: RedisCommand) -> Result<RedisResponse> {
        // Convert to internal operation
        let op = self.convert_command(cmd)?;
        
        // Apply safety checks
        self.validate_operation(&op)?;
        
        // Execute
        self.execute_operation(op).await
    }
}
```

## 5. Performance Considerations

### a. Compile-Time Optimization
- Pattern trie compilation
- Static validation caching
- Macro expansion optimization

### b. Runtime Performance
- Lock-free pattern matching
- Efficient layer routing
- Minimal validation overhead

## 6. Safety Guarantees

### a. Compile-Time Checks
- Pattern ownership validation
- Layer access rules
- Invalidation completeness

### b. Runtime Checks
- Race condition prevention
- Deadlock avoidance
- Resource limit enforcement