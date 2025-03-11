use core::prelude::*;
use stateless::cache_manifest;

#[test]
fn test_basic_ownership() {
    #[cache_manifest]
    struct TestCache {
        #[owns = "user:*"]
        user_data: UserCache,
        
        #[owns = "product:*"]
        product_data: ProductCache,
    }
    
    // This should compile
    #[cache(owns = "user:*")]
    async fn update_user() {
        // Valid ownership
    }
    
    // This should not compile
    // #[cache(owns = "user:*")]  // Ownership conflict
    // async fn another_user_owner() {}
}

#[test]
fn test_borrowing() {
    #[cache_manifest]
    struct TestCache {
        #[owns = "user:*"]
        user_data: UserCache,
    }
    
    // Multiple borrows should be fine
    #[cache(borrows = "user:*")]
    async fn read_user1() {}
    
    #[cache(borrows = "user:*")]
    async fn read_user2() {}
    
    // But can't borrow and own
    // #[cache(owns = "user:*")]  // Should fail
    // async fn write_user() {}
}

#[test]
fn test_pattern_specificity() {
    #[cache_manifest]
    struct TestCache {
        #[owns = "user:*"]
        user_data: UserCache,
    }
    
    // More specific pattern should work
    #[cache(owns = "user:123:profile")]
    async fn update_specific_profile() {}
    
    // But can't own parent pattern
    // #[cache(owns = "user:*")]  // Should fail
    // async fn own_all_users() {}
}

#[test]
fn test_layer_constraints() {
    #[cache_manifest]
    struct TestCache {
        #[client_primary]
        #[owns = "local:*"]
        client: ClientCache,
        
        #[edge_primary]
        #[owns = "edge:*"]
        edge: EdgeCache,
        
        #[server_primary]
        #[owns = "global:*"]
        server: ServerCache,
    }
    
    // Should enforce layer constraints
    #[cache(owns = "local:*")]
    async fn client_op() {}
    
    // #[cache(owns = "edge:*")]  // Should fail on client
    // async fn edge_op() {}
}

#[test]
fn test_invalidation() {
    #[cache_manifest]
    struct TestCache {
        #[owns = "user:*"]
        #[invalidates = "profile:*"]
        user_data: UserCache,
    }
    
    // Should track invalidation
    #[cache(owns = "user:*")]
    async fn update_user() {
        // Should automatically invalidate profile:*
    }
}

#[test]
fn test_lifetime_rules() {
    #[cache_manifest]
    struct TestCache {
        #[owns = "temp:*"]
        #[lifetime = "request"]
        temp_data: TempCache,
    }
    
    // Should enforce cleanup
    #[cache(owns = "temp:*")]
    async fn temp_operation() {
        // Should auto-cleanup after request
    }
}

#[test]
fn test_strategy_composition() {
    #[cache_manifest]
    struct TestCache {
        #[client_primary]
        #[fallback = "edge"]
        #[owns = "user:*"]
        user_data: UserStrategy,
    }
    
    // Should follow strategy
    #[cache(follows = "user_data")]
    async fn user_op() {
        // Should try client then edge
    }
} 