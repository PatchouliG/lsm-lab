use super::skip_list_imp::SkipListImp;

use crate::rand::simple_rand::Rand;
use std::sync::Arc;

pub struct SkipList<K: Copy + PartialOrd, V> {
    list: Arc<SkipListImp<K, V>>,
    r: Rand,
}

impl<K: Copy + PartialOrd, V: Copy> Clone for SkipList<K, V> {
    fn clone(&self) -> Self {
        SkipList {
            list: self.list.clone(),
            r: Rand::new(),
        }
    }
}

impl<K: Copy + PartialOrd, V: Copy> SkipList<K, V> {
    pub fn new() -> Self {
        SkipList {
            list: Arc::new(SkipListImp::new()),
            r: Rand::new(),
        }
    }
    pub fn add(&mut self, key: K, value: V) {
        self.list.add_internal(key, value, self.r.next() as usize);
    }
    pub fn get(&self, key: K) -> Option<V> {
        self.list.get(key)
    }
    pub fn remove(&mut self, key: K) {
        self.list.remove(key);
    }
}
