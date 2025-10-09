use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

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

impl<K, V> Default for InMemoryCache<K, V>
where
    K: Eq + std::hash::Hash + Clone + Send + Sync,
    V: Clone + Send + Sync,
{
    fn default() -> Self {
        Self::new()
    }
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
        self.map.get(key).map(|v| v.clone())
    }

    fn set(&self, key: K, value: V) {
        self.map.insert(key, value);
    }

    fn clear(&self) {
        self.map.clear();
    }
}

/// File-based cache that stores data in a directory.
/// Uses JSON serialization. Stores key-value pairs in files.
#[derive(Serialize, Deserialize)]
struct CacheEntry<K, V> {
    key: K,
    value: V,
}

pub struct FileCache<K, V> {
    dir: String,
    cache: Mutex<HashMap<K, V>>,
}

impl<K, V> FileCache<K, V>
where
    K: Eq + std::hash::Hash + Clone + Serialize + for<'de> Deserialize<'de> + Send + Sync,
    V: Clone + Serialize + for<'de> Deserialize<'de> + Send + Sync,
{
    pub fn new(dir: &str) -> Self {
        let mut cache = HashMap::new();
        if Path::new(dir).exists() {
            // Load existing cache from files
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension() == Some(std::ffi::OsStr::new("json")) {
                        if let Ok(content) = fs::read_to_string(&path) {
                            if let Ok(entry) = serde_json::from_str::<CacheEntry<K, V>>(&content) {
                                cache.insert(entry.key, entry.value);
                            }
                        }
                    }
                }
            }
        } else {
            fs::create_dir_all(dir).ok();
        }
        Self {
            dir: dir.to_string(),
            cache: Mutex::new(cache),
        }
    }

    fn key_to_filename(&self, key: &K) -> String {
        // Simple hash for filename
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hasher;
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        format!("{:x}.json", hasher.finish())
    }
}

impl<K, V> Cache<K, V> for FileCache<K, V>
where
    K: Eq + std::hash::Hash + Clone + Serialize + for<'de> Deserialize<'de> + Send + Sync,
    V: Clone + Serialize + for<'de> Deserialize<'de> + Send + Sync,
{
    fn get(&self, key: &K) -> Option<V> {
        let cache = self.cache.lock().unwrap();
        cache.get(key).cloned()
    }

    fn set(&self, key: K, value: V) {
        let mut cache = self.cache.lock().unwrap();
        cache.insert(key.clone(), value.clone());
        // Persist to file
        let filename = self.key_to_filename(&key);
        let file_path = Path::new(&self.dir).join(filename);
        let entry = CacheEntry { key, value };
        if let Ok(json) = serde_json::to_string(&entry) {
            fs::write(file_path, json).ok();
        }
    }

    fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
        fs::remove_dir_all(&self.dir).ok();
        fs::create_dir_all(&self.dir).ok();
    }
}

/// Example analysis result structure.
/// This can be customized based on what agents analyze.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub file_path: String,
    pub issues: Vec<String>,
    pub metrics: HashMap<String, f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_cache() {
        let cache = InMemoryCache::new();
        let key = "test.rs";
        let value = AnalysisResult {
            file_path: key.to_string(),
            issues: vec!["TODO".to_string()],
            metrics: [("loc".to_string(), 100.0)].into(),
        };
        cache.set(key.to_string(), value.clone());
        assert_eq!(cache.get(&key.to_string()), Some(value));
        cache.clear();
        assert_eq!(cache.get(&key.to_string()), None);
    }

    #[test]
    fn test_file_cache() {
        let temp_dir = tempfile::tempdir().unwrap();
        let cache_dir = temp_dir.path().to_string_lossy().to_string();
        let cache = FileCache::new(&cache_dir);
        let key = "test.rs".to_string();
        let value = AnalysisResult {
            file_path: key.clone(),
            issues: vec!["FIXME".to_string()],
            metrics: [("complexity".to_string(), 5.0)].into(),
        };
        cache.set(key.clone(), value.clone());
        // Simulate new instance to test persistence
        let cache2 = FileCache::new(&cache_dir);
        assert_eq!(cache2.get(&key), Some(value));
    }
}