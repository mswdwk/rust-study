use super::types::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Store {
    kv: HashMap<MemcachedKey, MemcachedValue>,
}

impl Store {
    pub fn new() -> Store {
        Store { kv: HashMap::new() }
    }

    pub fn with_capacity(size: usize) ->Store {
        Store { kv: HashMap::with_capacity(size)}
    }

    pub fn set(&mut self, key: &MemcachedKey, value: MemcachedValue) {
        self.kv.insert(key.clone(), value);
    }

    pub fn get(&mut self, key: &MemcachedKey) -> Option<&MemcachedValue> {
        self.kv.get(key)
    }

    pub fn remove(&mut self, key: &MemcachedKey) -> Option<MemcachedValue> {self.kv.remove(key)}
}
