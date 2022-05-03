use std::collections::hash_map::IntoIter;
use std::collections::HashMap;

use crate::common::{Key, Value};
use crate::log::{Log, LogEntry};

pub trait Memtable<K: Key, V: Value> {
    fn find(&self, key: &K) -> Option<&V>;
    fn add(&mut self, key: &K, value: V);
    fn delete(&mut self, key: &K);
    fn iter(self) -> MemTableIter<K, V>;
}

pub struct MemTableIter<K: Key, V: Value> {
    s: IntoIter<K, ValueWithStats<V>>,
}

impl<K: Key, V: Value> Iterator for MemTableIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.s.next();
        match res {
            None => None,
            Some(v) => {
                let k = v.0;
                match v.1 {
                    ValueWithStats::Delete => self.next(),
                    ValueWithStats::Add(value) => Option::Some((k, value)),
                }
            }
        }
    }
}

impl<K: Key, V: Value> MemTableIter<K, V> {
    fn new(m: HashMap<K, ValueWithStats<V>>) -> Self {
        let s = m.into_iter();
        MemTableIter { s }
    }
}

enum ValueWithStats<V: Value> {
    Add(V),
    Delete,
}

pub struct MemtableImp<K: Key, V: Value, L: Log<K, V>> {
    map: HashMap<K, ValueWithStats<V>>,
    log: L,
}

impl<K: Key, V: Value, L: Log<K, V>> MemtableImp<K, V, L> {
    pub fn new(l: L) -> Self {
        MemtableImp {
            map: HashMap::new(),
            log: l,
        }
    }
    pub fn clean() -> Self {
        todo!()
    }
}

impl<K: Key, V: Value, L: Log<K, V>> Memtable<K, V> for MemtableImp<K, V, L> {
    fn find(&self, key: &K) -> Option<&V> {
        let res = self.map.get(&key);
        match res {
            None => None,
            Some(v) => match v {
                ValueWithStats::Delete => None,
                ValueWithStats::Add(a) => Option::Some(a),
            },
        }
    }

    fn add(&mut self, key: &K, value: V) {
        self.map
            .insert(key.clone(), ValueWithStats::Add(value.clone()));
        self.log.add(LogEntry::Add(key.clone(), value.clone()))
    }

    fn delete(&mut self, key: &K) {
        self.map.insert(key.clone(), ValueWithStats::Delete);
        self.log.add(LogEntry::DELETE(key.clone()))
    }

    fn iter(self) -> MemTableIter<K, V> {
        MemTableIter::new(self.map)
    }
}

#[cfg(test)]
mod test {
    use crate::common::{Key, KeyImp, Value, ValueImp};
    use crate::log::MockLog;
    use crate::memtable::{Memtable, MemtableImp};

    #[test]
    fn test_memtable_get_and_set() {
        let mut memtable = MemtableImp::new(MockLog::new());

        let k = KeyImp::from_str("123");
        let v = ValueImp::from_str("123");
        // test insert
        memtable.add(&k, v);
        let res = memtable.find(&k);
        assert_eq!("123", res.unwrap().to_str());
        //     test override
        let v2 = ValueImp::from_str("234");
        memtable.add(&k, v2);
        let res = memtable.find(&k);
        assert_eq!("234", res.unwrap().to_str());

        //     test delete
        memtable.delete(&k);
        assert!(memtable.find(&k).is_none());
        //     test insert after delete
        let v2 = ValueImp::from_str("234");
        memtable.add(&k, v2);
        let res = memtable.find(&k);
        assert_eq!("234", res.unwrap().to_str());
    }

    #[test]
    fn test_memtable_iterator() {
        let mut memtable = MemtableImp::new(MockLog::new());

        let k = KeyImp::from_str("a");
        let mut v = ValueImp::from_str("a");
        memtable.add(&KeyImp::from_str("a"), ValueImp::from_str("a"));
        memtable.add(&KeyImp::from_str("b"), ValueImp::from_str("b"));
        memtable.add(&KeyImp::from_str("c"), ValueImp::from_str("c"));
        memtable.add(&KeyImp::from_str("d"), ValueImp::from_str("d"));
        memtable.delete(&KeyImp::from_str("c"));
        memtable.delete(&KeyImp::from_str("b"));
        memtable.delete(&KeyImp::from_str("a"));
        let mut it = memtable.iter();
        let (k, v) = it.next().unwrap();
        println!("{} {}", k, v.to_str());
        assert_eq!("d", k.to_str());
        assert_eq!("d", v.to_str());
    }
}
