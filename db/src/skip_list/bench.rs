use super::skip_list::SkipList;
#[derive(Copy, Clone)]
pub struct Config {
    pub read: f32,
    pub write: f32,
    pub delete: f32,
    pub init_size: usize,
    pub key_space: usize,
    pub operation_number: usize,
}

impl Config {
    pub fn new(
        read: f32,
        write: f32,
        delete: f32,
        init_size: usize,
        key_space: usize,
        operation_number: usize,
    ) -> Self {
        Config {
            read,
            write,
            delete,
            init_size,
            key_space,
            operation_number,
        }
    }
}

pub trait BenchList {
    fn add(&mut self, key: u32, value: u32);
    fn remove(&mut self, key: u32);
    fn get(&self, key: u32) -> Option<u32>;
}
use crate::rand::simple_rand::Rand;
use std::collections::HashMap;
use std::iter::Map;

pub fn setup<T: BenchList>(bench_list: &mut T, size: usize, mut r: &mut Rand, key_space: usize) {
    for i in 0..size {
        let key = (r.next() % key_space as u64) as u32;
        bench_list.add(key, key);
    }
}

pub fn single_thread_bench<T: BenchList>(bench_list: &mut T, c: Config, r: &mut Rand) {
    for i in 0..c.operation_number {
        let mut n = ((r.next() % 100) as f32) / 100.0;
        let key = (r.next() % c.key_space as u64) as u32;
        if n < c.read {
            bench_list.get(key);
            continue;
        }
        n -= c.read;
        if n < c.write {
            bench_list.add(key, key);
            continue;
        }
        bench_list.remove(key);
    }
}

fn multiple_thread_bench(c: Config, thread_number: usize) {}

impl BenchList for SkipList<u32, u32> {
    fn add(&mut self, key: u32, value: u32) {
        self.add(key, value);
    }

    fn remove(&mut self, key: u32) {
        self.remove(key);
    }

    fn get(&self, key: u32) -> Option<u32> {
        self.get(key)
    }
}

impl BenchList for HashMap<u32, u32> {
    fn add(&mut self, key: u32, value: u32) {
        self.insert(key, value);
    }

    fn remove(&mut self, key: u32) {
        self.remove(&key);
    }

    fn get(&self, key: u32) -> Option<u32> {
        let res = self.get(&key);
        res.map(|n| *n)
    }
}

mod test {
    use crate::rand::simple_rand::Rand;
    use crate::skip_list::bench::{setup, single_thread_bench, Config};
    use crate::skip_list::skip_list::SkipList;
    use std::collections::HashMap;
    use std::time::UNIX_EPOCH;

    #[test]
    fn test() {
        let mut list = SkipList::new();
        let config = Config {
            write: 0.1,
            read: 0.8,
            delete: 0.1,
            init_size: 100,
            key_space: 1000,
            operation_number: 100000,
        };
        let mut r = Rand::new();
        setup(&mut list, config.init_size, &mut r, config.key_space);
        let a = std::time::SystemTime::now();
        let since_the_epoch = a
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        println!("start {}", since_the_epoch);
        single_thread_bench(&mut list, config, &mut r);
        let b = std::time::SystemTime::now();
        let since_the_epoch = b
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        println!("end {}", since_the_epoch);
    }

    #[test]
    fn testHashMap() {
        let mut list = HashMap::new();
        let config = Config {
            write: 0.1,
            read: 0.8,
            delete: 0.1,
            init_size: 100,
            key_space: 1000,
            operation_number: 1000,
        };
        let mut r = Rand::new();
        setup(&mut list, config.init_size, &mut r, config.key_space);
        let a = std::time::SystemTime::now();
        let since_the_epoch = a
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        println!("start {}", since_the_epoch);
        single_thread_bench(&mut list, config, &mut r);
        let b = std::time::SystemTime::now();
        let since_the_epoch = b
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        println!("end {}", since_the_epoch);
    }
}
