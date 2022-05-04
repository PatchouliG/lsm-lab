use crate::common::{Key, Value};

pub trait Log {
    fn add(&mut self, entry: Vec<u8>);
}

pub struct MockLog {}

impl MockLog {
    pub fn new() -> Self {
        MockLog {}
    }
}

impl Log for MockLog {
    fn add(&mut self, entry: Vec<u8>) {
        todo!()
    }
}
