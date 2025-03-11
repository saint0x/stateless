use core::prelude::*;
use stateless::{cache_manifest, Cache};
use crate::common::{init, runtime};

#[tokio::test]
async fn test_user_profile_scenario() {
    #[cache_manifest]
    struct UserCache {
        #[client_primary]
        #[owns = "user:*:profile"]
        profile: ProfileCache,
        
        #[edge_primary]
        #[owns = "user:*:preferences"]
        preferences: PreferencesCache,
        
        #[server_primary]
        #[owns = "user:*:analytics"]
        analytics: AnalyticsCache,
    }
    
    let cache = stateless::quick_start();
    
    // Update profile (should be client-side)
    #[cache(owns = "user:*:profile")]
    async fn update_profile(id: &str, data: &str) {
        cache.set(&format!("user:{}:profile", id), data).await?;
    }
    
    // Read preferences (should hit edge)
    #[cache(borrows = "user:*:preferences")]
    async fn get_preferences(id: &str) -> Option<String> {
        cache.get(&format!("user:{}:preferences", id)).await?
    }
    
    // Update analytics (should be server-side)
    #[cache(owns = "user:*:analytics")]
    async fn log_analytics(id: &str, event: &str) {
        cache.set(&format!("user:{}:analytics", id), event).await?;
    }
    
    // Execute scenario
    update_profile("123", "Alice").await?;
    get_preferences("123").await?;
    log_analytics("123", "login").await?;
}

#[tokio::test]
async fn test_offline_scenario() {
    #[cache_manifest]
    struct OfflineCache {
        #[client_primary]
        #[with_offline]
        #[owns = "offline:*"]
        offline_data: OfflineStrategy,
    }
    
    let cache = stateless::quick_start();
    
    // Should work offline
    cache.set("offline:doc1", "content").await?;
    assert_eq!(cache.get("offline:doc1").await?, Some("content".into()));
    
    // Simulate offline
    cache.simulate_offline();
    assert_eq!(cache.get("offline:doc1").await?, Some("content".into()));
    
    // Should sync when back online
    cache.simulate_online();
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    // Verify sync happened
}

#[tokio::test]
async fn test_edge_caching_scenario() {
    #[cache_manifest]
    struct ContentCache {
        #[edge_primary]
        #[ttl = "1h"]
        #[owns = "content:*"]
        content: EdgeStrategy,
    }
    
    let cache = stateless::quick_start();
    
    // Should cache at edge
    cache.set("content:page1", "data").await?;
    
    // Should serve from nearest edge
    let value = cache.get("content:page1")
        .from_region("us-east")
        .await?;
    
    assert_eq!(value, Some("data".into()));
}

#[tokio::test]
async fn test_redis_compatibility() {
    let cache = stateless::redis_compatible();
    
    // Should support Redis commands
    redis::cmd("SET")
        .arg("key")
        .arg("value")
        .query_async(&mut cache)
        .await?;
        
    let value: String = redis::cmd("GET")
        .arg("key")
        .query_async(&mut cache)
        .await?;
        
    assert_eq!(value, "value");
}

#[tokio::test]
async fn test_transaction_scenario() {
    #[cache_manifest]
    struct BankCache {
        #[server_primary]
        #[owns = "account:*"]
        accounts: AccountStrategy,
    }
    
    let cache = stateless::quick_start();
    
    // Should handle transactions
    cache.atomic_batch(|tx| {
        tx.incr("account:123", 100)?;  // Deposit
        tx.decr("account:456", 100)?;  // Withdraw
        Ok(())
    }).await?;
    
    assert_eq!(cache.get("account:123").await?, Some(100));
    assert_eq!(cache.get("account:456").await?, Some(-100));
}

#[tokio::test]
async fn test_pattern_invalidation() {
    #[cache_manifest]
    struct CartCache {
        #[client_primary]
        #[owns = "cart:*"]
        #[invalidates = "total:*"]
        cart: CartStrategy,
    }
    
    let cache = stateless::quick_start();
    
    // Update cart
    cache.set("cart:123:item1", "product1").await?;
    cache.set("cart:123:item2", "product2").await?;
    
    // Total should be invalidated
    assert!(cache.get("total:123").await?.is_none());
    
    // Recalculate total
    cache.set("total:123", "300").await?;
    
    // Update cart again
    cache.set("cart:123:item3", "product3").await?;
    
    // Total should be invalidated again
    assert!(cache.get("total:123").await?.is_none());
} 