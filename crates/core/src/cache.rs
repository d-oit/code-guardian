use dashmap::DashMap;

/// Trait for caching analysis results.
/// Key is typically a file path or module identifier.
/// Value is the serialized analysis result.
pub trait Cache<K, V>: Send + Sync
where
    K: Eq + std::hash::Hash + Clone + Send + Sync,
    V: Clone + Send + Sync,
{
    fn get(&self, key: &K) -> Option<V>;
    fn set(&self, key: K, value: V);
    fn clear(&self);
}

/// In-memory cache using DashMap for concurrent access.
pub struct InMemoryCache<K, V> {
    map: DashMap<K, V>,
}

impl<K, V> InMemoryCache<K, V>
where
    K: Eq + std::hash::Hash + Clone + Send + Sync,
    V: Clone + Send + Sync,
{
    pub fn new() -> Self {
        Self {
            map: DashMap::new(),
        }
    }
}

impl<K, V> Cache<K, V> for InMemoryCache<K, V>
where
    K: Eq + std::hash::Hash + Clone + Send + Sync,
    V: Clone + Send + Sync,
{
    fn get(&self, key: &K) -> Option<V> {
        self.map.get(key).map(|value| value.clone())
    }

    fn set(&self, key: K, value: V) {
        self.map.insert(key, value);
    }

    fn clear(&self) {
        self.map.clear();
    }
}

impl<K, V> Default for InMemoryCache<K, V>
where
    K: Eq + std::hash::Hash + Clone + Send + Sync,
    V: Clone + Send + Sync,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_cache_basic_operations() {
        let cache: InMemoryCache<String, String> = InMemoryCache::new();

        // Test initial empty state
        assert!(cache.get(&"key1".to_string()).is_none());

        // Test set and get
        cache.set("key1".to_string(), "value1".to_string());
        assert_eq!(cache.get(&"key1".to_string()), Some("value1".to_string()));

        // Test overwrite
        cache.set("key1".to_string(), "value2".to_string());
        assert_eq!(cache.get(&"key1".to_string()), Some("value2".to_string()));

        // Test multiple keys
        cache.set("key2".to_string(), "value3".to_string());
        assert_eq!(cache.get(&"key1".to_string()), Some("value2".to_string()));
        assert_eq!(cache.get(&"key2".to_string()), Some("value3".to_string()));
    }

    #[test]
    fn test_cache_clear() {
        let cache: InMemoryCache<String, String> = InMemoryCache::new();

        cache.set("key1".to_string(), "value1".to_string());
        cache.set("key2".to_string(), "value2".to_string());

        assert!(cache.get(&"key1".to_string()).is_some());
        assert!(cache.get(&"key2".to_string()).is_some());

        cache.clear();

        assert!(cache.get(&"key1".to_string()).is_none());
        assert!(cache.get(&"key2".to_string()).is_none());
    }

    #[test]
    fn test_cache_with_different_types() {
        let cache: InMemoryCache<i32, Vec<String>> = InMemoryCache::new();

        let value = vec!["item1".to_string(), "item2".to_string()];
        cache.set(42, value.clone());

        assert_eq!(cache.get(&42), Some(value));
        assert!(cache.get(&43).is_none());
    }

    #[test]
    fn test_cache_default() {
        let cache: InMemoryCache<String, i32> = InMemoryCache::default();
        assert!(cache.get(&"test".to_string()).is_none());

        cache.set("test".to_string(), 100);
        assert_eq!(cache.get(&"test".to_string()), Some(100));
    }

    #[test]
    fn test_cache_concurrent_access() {
        use std::sync::Arc;
        use std::thread;

        let cache: Arc<InMemoryCache<String, String>> = Arc::new(InMemoryCache::new());
        let mut handles = vec![];

        // Spawn multiple threads to test concurrent access
        for i in 0..10 {
            let cache_clone = Arc::clone(&cache);
            let handle = thread::spawn(move || {
                let key = format!("key_{}", i);
                let value = format!("value_{}", i);
                cache_clone.set(key.clone(), value.clone());
                let retrieved = cache_clone.get(&key);
                assert_eq!(retrieved, Some(value));
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // Verify all values are present
        for i in 0..10 {
            let key = format!("key_{}", i);
            let expected = format!("value_{}", i);
            assert_eq!(cache.get(&key), Some(expected));
        }
    }
}
