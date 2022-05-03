use std::marker::PhantomData;

use crate::common::{Key, Value};
use crate::level::Level;
use crate::memtable::Memtable;
use crate::sstable::SStable;

trait LsmMeta {
    fn apply_meta_change(change: MetaChange);
}

enum MetaChange {}

struct Lsm<
    K: Key,
    V: Value,
    M: Memtable<K, V>,
    SS: SStable<K, V>,
    L: Level<K, SS = SS>,
    LM: LsmMeta,
> {
    memtable: M,
    levels: Vec<L>,
    meta: LM,
    k: PhantomData<K>,
    v: PhantomData<V>,
}

impl<K: Key, V: Value, M: Memtable<K, V>, SS: SStable<K, V>, L: Level<K, SS = SS>, LM: LsmMeta>
    Lsm<K, V, M, SS, L, LM>
{
    // pub
    pub fn from_meta(meta: LM) -> Self {
        todo!()
    }
    pub fn add() {}
    pub fn delete() {}
    pub fn get() {}

    // compact
    fn compact_memtable_with_sstable() {
        //     build sstalbe from memtable and overlap sstable at level 1
        //     add sstable to level 1,remove input sstable
        //     clean memtable
        //     change meta
    }
    // compact level n to level n+1 (n>0)
    fn compact_sstables_in_level(level: usize) {
        //     pick next sstable in level n
        // find sstable in level n+1 range overlap with n
        // build new sstable from compact
        // insert and delete sstable
        // change metadata
    }

    //     find
    fn find_in_memtable(&self, key: &K) -> Option<&V> {
        self.memtable.find(key)
    }
    fn find_in_level(&self, key: &K, level_number: usize) -> Option<&V> {
        let level = self.levels.get(level_number).unwrap();
        let ss = level.get_sstable_contains(key);
        match ss {
            None => None,
            Some(ss1) => ss1.find(key),
        }
    }
    //
}
