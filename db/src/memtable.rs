use std::collections::hash_map::IntoIter;
use std::collections::HashMap;

use crate::common::{Key, Value};
use crate::log::Log;

pub struct MemTableIter {
    s: IntoIter<Key, ValueWithStats>,
}

impl Iterator for MemTableIter {
    type Item = (Key, Value);

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

enum ValueWithStats {
    Add(Value),
    Delete,
}

pub struct MemTable<L: Log> {
    map: HashMap<Key, ValueWithStats>,
    log: L,
}

impl MemTableIter {
    fn new(m: HashMap<Key, ValueWithStats>) -> Self {
        let s = m.into_iter();
        MemTableIter { s }
    }
}

impl<L: Log> MemTable<L> {
    pub fn new(l: L) -> Self {
        MemTable {
            map: HashMap::new(),
            log: l,
        }
    }

    pub fn clean() -> Self {
        todo!()
    }

    fn restore_from_log(&self, l: L) -> Self {
        todo!()
    }
    fn find(&self, key: &Key) -> Option<&Value> {
        let res = self.map.get(&key);
        match res {
            None => None,
            Some(v) => match v {
                ValueWithStats::Delete => None,
                ValueWithStats::Add(a) => Option::Some(&a),
            },
        }
    }

    fn add(&mut self, key: &Key, value: Value) {
        self.map
            .insert(key.clone(), ValueWithStats::Add(value.clone()));
        // todo
        // self.log.add(LogEntry::Add(key.clone(), value.clone()))
    }

    fn delete(&mut self, key: &Key) {
        self.map.insert(key.clone(), ValueWithStats::Delete);
        // todo
        // self.log.add(LogEntry::DELETE(key.clone()))
    }

    fn iter(self) -> MemTableIter {
        MemTableIter::new(self.map)
    }
}

#[cfg(test)]
mod test {
    use crate::common::{Key, Value};
    use crate::log::MockLog;
    use crate::memtable::MemTable;

    #[test]
    fn test_memtable_get_and_set() {
        let mut memtable = MemTable::new(MockLog::new());

        let k = Key::from_str("123");
        let v = Value::from_str("123");
        // test insert
        memtable.add(&k, v);
        let res = memtable.find(&k);
        assert_eq!("123", res.unwrap().to_str());
        //     test override
        let v2 = Value::from_str("234");
        memtable.add(&k, v2);
        let res = memtable.find(&k);
        assert_eq!("234", res.unwrap().to_str());

        //     test delete
        memtable.delete(&k);
        assert!(memtable.find(&k).is_none());
        //     test insert after delete
        let v2 = Value::from_str("234");
        memtable.add(&k, v2);
        let res = memtable.find(&k);
        assert_eq!("234", res.unwrap().to_str());
    }

    #[test]
    fn test_memtable_iterator() {
        let mut memtable = MemTable::new(MockLog::new());

        let k = Key::from_str("a");
        let mut v = Value::from_str("a");
        memtable.add(&Key::from_str("a"), Value::from_str("a"));
        memtable.add(&Key::from_str("b"), Value::from_str("b"));
        memtable.add(&Key::from_str("c"), Value::from_str("c"));
        memtable.add(&Key::from_str("d"), Value::from_str("d"));
        memtable.delete(&Key::from_str("c"));
        memtable.delete(&Key::from_str("b"));
        memtable.delete(&Key::from_str("a"));
        let mut it = memtable.iter();
        let (k, v) = it.next().unwrap();
        println!("{} {}", k, v.to_str());
        assert_eq!("d", k.to_str());
        assert_eq!("d", v.to_str());
    }
}
