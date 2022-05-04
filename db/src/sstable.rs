use std::cell::RefMut;
use std::cmp::Ordering;
use std::io::{BufWriter, Cursor, Write};
use std::marker::PhantomData;

use crate::common::Key;
use crate::common::{encode_kv, Value};
use crate::meta_manager::MetaManager;
use crate::storage::{BlockMeta, StorageReader, StorageWriter};

// todo sstable file gc
// a block based , immutable ,ordered list
pub trait SStable<M: MetaManager> {
    fn from_compact_sstables(meta: M, sstables: &dyn Iterator<Item = (Key, Value)>) -> Self;
    fn range_contain(&self, key: &Key) -> bool;
    fn find(&self, key: &Key) -> Option<&Value>;
    fn to_iter(self) -> Box<dyn Iterator<Item = (Key, Value)>>;
}

// // sstable struct
//     |block 1|
//     |block 2|
//     ........
//     |block N|
//     _________
//     |meta block|
// //
struct SStableImp<SR: StorageReader> {
    storage_reader: SR,
    // sorted
    block_metas: Vec<BlockMeta>,
}

const FILE_SIZE: usize = 4 * 1024 * 1024;
const BLOCK_SIZE: usize = 4 * 1024;
// impl Write for [u32;BLOCK_SIZE]{
//     fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//         todo!()
//     }
//
//     fn flush(&mut self) -> std::io::Result<()> {
//         todo!()
//     }
// }
impl<SR: StorageReader> SStableImp<SR> {
    pub fn new(storage_reader: SR) -> Self {
        // read and decode block meta from end
        todo!()
    }
    fn encode_block_meta(metas: &Vec<BlockMeta>) -> Vec<u8> {
        todo!()
    }
    fn decode_block_meta(v: &Vec<u8>) -> Vec<BlockMeta> {
        todo!()
    }
    fn build_sstable<MM: MetaManager, SW: StorageWriter>(
        mut iterator: Box<dyn Iterator<Item = (Key, Value)>>,
        mm: MM,
        storage_writer: SW,
    ) {
        //<editor-fold desc="todo">
        // let metas: Vec<BlockMeta<K>> = Vec::new();
        // let mut data_block = [0; BLOCK_SIZE];
        // let size: usize = 0;
        // // get encode date from iterator
        // match iterator.next() {
        //     None => {}
        //     Some((k, v)) => encode_kv(k, v, &mut data_block),
        // }
        // 1.write data to data_block
        // 2. write data_block to storage writer if is full, and clean data_block
        // 3. check if read block size limit ,if is,go to 5
        // 4. go to 1.
        // 5. write metablock to end of storage write
        // 6. update metaManager
        //</editor-fold>
    }
    fn find(&self, k: &Key) -> Option<Value> {
        // get block
        // check block in cache, if not found get from storage ,decode it and update cache
        // binary search
        todo!()
    }
}

struct SStableImpIter {}
impl Iterator for SStableImpIter {
    type Item = (Key, Value);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

// impl<K: Key, V: Value, MM: MetaManager> SStable<K, V, MM> for SStableImp<K, V, MM> {
//     fn from_compact_sstables(meta: MM, sstables: &dyn KV_iterator<K, V>) -> Self {
//         todo!()
//     }
//
//     fn range_contain(&self, key: &Key) -> bool {
//         assert!(self.block_metas.len() > 0);
//         let start = &self.block_metas.first().unwrap().start();
//         let end = &self.block_metas.last().unwrap().end();
//         key.ge(start) && key.lt(end)
//     }
//
//     fn find(&self, key: &Key) -> Option<&V> {
//         todo!()
//     }
//
//     fn to_iter(self) -> Box<dyn Iterator<Item = (K, V)>> {
//         todo!()
//     }
// }

// impl<K: Key, V: Value, IT: Iterator<Item = (K, V)>> SStable<K, V, IT> for SStableImp<K, V> {
//     fn from_iterator(iterator: IT) -> Self {
//         todo!()
//     }
//
//     fn range_contain(&self, key: &Key) -> bool {
//         todo!()
//     }
//
//     fn find(&self, key: &Key) -> Option<&V> {
//         todo!()
//     }
// }

// impl<K: Key, V: Value, S: StorageReader> SStable<K, V> for SStableImp<K, V, S> {
//     fn from_compact(sstables: Vec<&Self>) -> Self {
//         todo!()
//     }
//
//     fn from_iterator(sstable_file: Self) -> Self {
//         todo!()
//     }
//
//     fn range_contain(&self, key: &Key) -> bool {
//         todo!()
//     }
//
//     fn find(&self, key: &Key) -> Option<&V> {
//         todo!()
//     }
// }

// impl<K: Key, V: Value, S: StorageReader> SStableImp<K, V, S> {
//     fn find_block(&self, key: K) -> Option<BlockMeta<K>> {
//         todo!()
//     }
// }

#[cfg(test)]
mod test {
    use crate::common::{Key, Value};
    use crate::sstable::SStable;
}
