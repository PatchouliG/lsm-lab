use crate::common::Key;

pub trait Level {
    type SS;
    fn get_sstable_contains(&self, key: &Key) -> Option<&Self::SS>;
    fn length(&self) -> usize;
    fn get_sstables_in_range(&self, start_key: &Key, end_key: &Key) -> Vec<&Self::SS>;
    fn delete_sstables_in_range(&self, start_key: &Key, end_key: &Key);
    fn add_sstable(&mut self, sstable: Self::SS);
}
