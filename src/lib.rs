use std::collections::HashMap;
use std::collections::LinkedList;

pub struct LruCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    list: LinkedList<K>,
}

impl<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Copy,
{
    pub fn new(capacity: usize) -> Self {
        LruCache {
            capacity,
            map: HashMap::new(),
            list: LinkedList::new(),
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            // Key already exists, update the value and move it to the front of the list
            self.map.insert(key, value);
            self.list.retain(|&x| x != key);
            self.list.push_front(key);
        } else {
            // Key does not exist, insert it
            if self.map.len() >= self.capacity {
                // Cache is full, evict the least recently used item
                if let Some(oldest_key) = self.list.pop_back() {
                    self.map.remove(&oldest_key);
                }
            }
            self.map.insert(key, value);
            self.list.push_front(key);
        }
    }

    pub fn get(&mut self, key: K) -> Option<&V> {
        if self.map.contains_key(&key) {
            // Key exists, move it to the front of the list
            self.list.retain(|&x| x != key);
            self.list.push_front(key);
            self.map.get(&key)
        } else {
            None
        }
    }
}
