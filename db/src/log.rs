use crate::common::{Key, Value};

pub trait Log<K: Key, V: Value> {
    fn add(&mut self, entry: LogEntry<K, V>);
}

pub enum LogEntry<K: Key, V: Value> {
    Add(K, V),
    DELETE(K),
}

pub struct MockLog {}

impl MockLog {
    pub fn new() -> Self {
        MockLog {}
    }
}

impl<K: Key, V: Value> Log<K, V> for MockLog {
    fn add(&mut self, entry: LogEntry<K, V>) {
        //     nothing
    }
}
