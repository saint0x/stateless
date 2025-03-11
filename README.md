# stateless

A universal caching layer with compile-time safety guarantees.

## Thesis

Stateless introduces a novel approach to distributed caching by applying Rust's borrow checker principles to cache operations. This ensures:

1. **Compile-Time Safety**: Cache conflicts are caught before runtime
2. **Zero Runtime Overhead**: Safety checks happen during compilation
3. **Cross-Platform Consistency**: Same safety guarantees across client, edge, and server

## Example

```rust
#[cache_manifest]
struct UserCache {
    #[owns = "user:*"]
    #[invalidates = "profile:*"]
    user_data: UserStore,
}

// Compile-time guarantee: No other function can write to user:* while this runs
#[cache(owns = "user:*")]
async fn update_user() {
    // Safe to modify user data
}

// Compile-time guarantee: Can read user:* concurrently
#[cache(borrows = "user:*")]
async fn read_user() {
    // Safe to read user data
}
```

## Status

🚧 Early development - Not ready for production use 