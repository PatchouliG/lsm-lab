use std::marker::PhantomData;

use crate::common::Key;
use crate::common::Value;
use crate::storage::StorageReader;
use std::cmp::Ordering;

trait Log<K: Key, V: Value> {
    fn append(&mut self, key: K, value: V);
    fn flush(&mut self);
}

// todo sstable file gc

pub trait SStable<K: Key, V: Value> {
    fn from_compact(sstables: Vec<&Self>) -> Self;
    fn from_iterator(sstable_file: Self) -> Self;
    fn range_contain(&self, key: &K) -> bool;
    fn find(&self, key: &K) -> Option<&V>;
}

struct BlockMeta<K: Key> {
    start: K,
    end: K,
    offset: usize,
}

impl<K: Key> BlockMeta<K> {
    pub fn range(&self) -> (&K, &K) {
        (&self.start, &self.end)
    }
    pub fn offset(&self) -> usize {
        self.offset
    }
}

impl<K: Key> PartialEq for BlockMeta<K> {
    fn eq(&self, other: &Self) -> bool {
        self.start.eq(&other.start) && (self.end.eq(&other.end))
    }
}

impl<K: Key> PartialOrd for BlockMeta<K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.start.partial_cmp(&other.start)
    }
}

struct SStableImp<K: Key, V: Value, SR: StorageReader> {
    // sort by key
    block_meta: Vec<BlockMeta<K>>,
    storage: SR,
    k: PhantomData<K>,
    v: PhantomData<V>,
}

impl<K: Key, V: Value, S: StorageReader> SStable<K, V> for SStableImp<K, V, S> {
    fn from_compact(sstables: Vec<&Self>) -> Self {
        todo!()
    }

    fn from_iterator(sstable_file: Self) -> Self {
        todo!()
    }

    fn range_contain(&self, key: &K) -> bool {
        todo!()
    }

    fn find(&self, key: &K) -> Option<&V> {
        todo!()
    }
}

impl<K: Key, V: Value, S: StorageReader> SStableImp<K, V, S> {
    fn find_block(&self, key: K) -> Option<BlockMeta<K>> {
        todo!()
    }
}
//
// impl<K: Key, V: Value> IntoIterator for SStableImp<K, V> {
//     type Item = (K, V);
//     type IntoIter = SSTableIter<K, V>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }

struct SSTableIter<K: Key, V: Value> {
    k: PhantomData<K>,
    v: PhantomData<V>,
}

impl<K: Key, V: Value> Iterator for SSTableIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
