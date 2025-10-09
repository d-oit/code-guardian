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

