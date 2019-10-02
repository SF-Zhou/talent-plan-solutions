use std::collections::HashMap;

/// The `KvStore` stores key-value pairs in-memory
/// Examples
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// let key = String::from("key");
/// let value = String::from("value");
/// store.set(key.clone(), value.clone());
/// assert_eq!(store.get(key.clone()).unwrap(), value.clone());
/// ```
#[derive(Default)]
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}
