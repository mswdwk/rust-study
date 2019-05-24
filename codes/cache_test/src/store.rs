use super::types::*;
use super::errors::*;

use std::collections::HashMap;


#[derive(Debug)]
pub struct MemStore {
    kv: HashMap<MemcachedKey, MemcachedValue>,
}

impl MemStore {
    pub fn new() -> MemStore {
        MemStore { kv: HashMap::new() }
    }

    pub fn with_capacity(size: usize) ->MemStore {
        MemStore { kv: HashMap::with_capacity(size)}
    }

    pub fn set(&mut self, key: &MemcachedKey, value: MemcachedValue) {
        self.kv.insert(key.clone(), value);
    }

    pub fn get(&mut self, key: &MemcachedKey) -> Option<&MemcachedValue> {
        self.kv.get(key)
    }

    pub fn remove(&mut self, key: &MemcachedKey) -> Option<MemcachedValue> {self.kv.remove(key)}
}

pub trait DbStore {
    fn create() -> Result<()>;
    fn get(key: &MemcachedKey) ->Result<Option<MemcachedKey>>;
    fn delete();
}