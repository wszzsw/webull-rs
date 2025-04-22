use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Cache entry.
#[derive(Debug, Clone)]
struct CacheEntry<T> {
    /// Cached value
    value: T,

    /// When the entry was created
    created_at: Instant,

    /// Time-to-live for the entry
    ttl: Duration,
}

impl<T> CacheEntry<T> {
    /// Create a new cache entry.
    fn new(value: T, ttl: Duration) -> Self {
        Self {
            value,
            created_at: Instant::now(),
            ttl,
        }
    }

    /// Check if the entry is expired.
    fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.ttl
    }
}

/// Cache key.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CacheKey {
    /// Method (GET, POST, etc.)
    method: String,

    /// URL
    url: String,

    /// Query parameters
    query: Option<String>,

    /// Request body
    body: Option<String>,
}

impl CacheKey {
    /// Create a new cache key.
    fn new(method: &str, url: &str, query: Option<&str>, body: Option<&str>) -> Self {
        Self {
            method: method.to_string(),
            url: url.to_string(),
            query: query.map(|s| s.to_string()),
            body: body.map(|s| s.to_string()),
        }
    }
}

/// Response cache.
pub struct ResponseCache<T: Clone + Send + Sync> {
    /// Cached responses
    cache: Mutex<HashMap<CacheKey, CacheEntry<T>>>,

    /// Default time-to-live for cache entries
    default_ttl: Duration,

    /// Maximum number of entries in the cache
    max_entries: usize,
}

impl<T: Clone + Send + Sync> ResponseCache<T> {
    /// Create a new response cache.
    pub fn new(default_ttl: Duration, max_entries: usize) -> Self {
        Self {
            cache: Mutex::new(HashMap::new()),
            default_ttl,
            max_entries,
        }
    }

    /// Get a cached response.
    pub fn get(
        &self,
        method: &str,
        url: &str,
        query: Option<&str>,
        body: Option<&str>,
    ) -> Option<T> {
        let key = CacheKey::new(method, url, query, body);
        let mut cache = self.cache.lock().unwrap();

        if let Some(entry) = cache.get(&key) {
            if entry.is_expired() {
                // Remove expired entry
                cache.remove(&key);
                None
            } else {
                // Return cached value
                Some(entry.value.clone())
            }
        } else {
            None
        }
    }

    /// Store a response in the cache.
    pub fn set(
        &self,
        method: &str,
        url: &str,
        query: Option<&str>,
        body: Option<&str>,
        value: T,
        ttl: Option<Duration>,
    ) {
        let key = CacheKey::new(method, url, query, body);
        let ttl = ttl.unwrap_or(self.default_ttl);
        let entry = CacheEntry::new(value, ttl);

        let mut cache = self.cache.lock().unwrap();

        // Check if we need to evict entries
        if cache.len() >= self.max_entries {
            // Remove expired entries first
            let expired_keys: Vec<_> = cache
                .iter()
                .filter(|(_, entry)| entry.is_expired())
                .map(|(key, _)| key.clone())
                .collect();

            for key in expired_keys {
                cache.remove(&key);
            }

            // If we still need to evict entries, remove the oldest ones
            if cache.len() >= self.max_entries {
                // Get all entries
                let entries: Vec<_> = cache.iter().collect();

                // Sort by creation time
                let mut sorted_entries: Vec<_> = entries.iter().collect();
                sorted_entries.sort_by_key(|(_, entry)| entry.created_at);

                // Calculate how many to remove
                let to_remove = entries.len() - self.max_entries + 1;

                // Remove the oldest entries
                let keys_to_remove: Vec<_> = sorted_entries
                    .iter()
                    .take(to_remove)
                    .map(|(k, _)| (*k).clone())
                    .collect();
                for key in keys_to_remove {
                    cache.remove(&key);
                }
            }
        }

        // Add the new entry
        cache.insert(key, entry);
    }

    /// Clear the cache.
    pub fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }

    /// Remove expired entries from the cache.
    pub fn cleanup(&self) {
        let mut cache = self.cache.lock().unwrap();
        let expired_keys: Vec<_> = cache
            .iter()
            .filter(|(_, entry)| entry.is_expired())
            .map(|(key, _)| key.clone())
            .collect();

        for key in expired_keys {
            cache.remove(&key);
        }
    }
}

/// Cache manager for API responses.
pub struct CacheManager {
    /// Response caches for different types
    caches: Mutex<HashMap<String, Arc<dyn Any + Send + Sync>>>,
}

impl CacheManager {
    /// Create a new cache manager.
    pub fn new() -> Self {
        Self {
            caches: Mutex::new(HashMap::new()),
        }
    }

    /// Get a cache for a specific type.
    pub fn get_cache<T: Clone + Send + Sync + 'static>(&self, name: &str) -> Arc<ResponseCache<T>> {
        let mut caches = self.caches.lock().unwrap();

        // Check if the cache exists
        if let Some(cache) = caches.get(name) {
            // Try to downcast to the correct type
            if let Some(typed_cache) = cache.clone().downcast_arc::<ResponseCache<T>>().ok() {
                return typed_cache;
            }
        }

        // Create a new cache
        let cache = Arc::new(ResponseCache::<T> {
            cache: Mutex::new(HashMap::new()),
            default_ttl: Duration::from_secs(60),
            max_entries: 1000,
        });

        // Store the cache
        caches.insert(
            name.to_string(),
            cache.clone() as Arc<dyn Any + Send + Sync>,
        );

        cache
    }

    /// Clear all caches.
    pub fn clear_all(&self) {
        let mut caches = self.caches.lock().unwrap();
        caches.clear();
    }
}

use std::any::{Any, TypeId};

/// Extension trait for Arc<dyn Any + Send + Sync>.
trait ArcAnyExt {
    /// Downcast to a specific type.
    fn downcast_arc<T: 'static>(self) -> Result<Arc<T>, Self>
    where
        Self: Sized;
}

impl ArcAnyExt for Arc<dyn Any + Send + Sync> {
    fn downcast_arc<T: 'static>(self) -> Result<Arc<T>, Self> {
        if (*self).type_id() == TypeId::of::<T>() {
            // SAFETY: We just checked that the type is correct
            let ptr = Arc::into_raw(self) as *const T;
            // SAFETY: We're creating a new Arc from the raw pointer
            Ok(unsafe { Arc::from_raw(ptr) })
        } else {
            Err(self)
        }
    }
}
