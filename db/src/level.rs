use crate::common::Key;

pub trait Level<K: Key> {
    type SS;
    fn get_sstable_contains(&self, key: &K) -> Option<&Self::SS>;
    fn length(&self) -> usize;
    fn get_sstables_in_range(&self, start_key: &K, end_key: &K) -> Vec<&Self::SS>;
    fn delete_sstables_in_range(&self, start_key: &K, end_key: &K);
    fn add_sstable(&mut self, sstable: Self::SS);
}
