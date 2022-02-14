extern crate db;
use std::collections::HashMap;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

use crate::db::rand::simple_rand::Rand;
use crate::db::skip_list::skip_list::SkipList;
use db::skip_list::bench::Config;
use std::time::Duration;

fn bench_skipList_single_thread(c: &mut Criterion) {
    let mut group = c.benchmark_group("My Group");

    let mut list = SkipList::new();
    let mut hashMap_list = HashMap::new();
    let mut r = Rand::new();
    let config = Config::new(0.1, 0.8, 0.1, 1000, 10000, 100000);
    crate::db::skip_list::bench::setup(&mut list, config.init_size, &mut r, config.key_space);
    crate::db::skip_list::bench::setup(
        &mut hashMap_list,
        config.init_size,
        &mut r,
        config.key_space,
    );

    group.bench_function("skip list single thread", |b| {
        b.iter(|| {
            crate::db::skip_list::bench::single_thread_bench(&mut list, config, &mut r);
        });
    });
    group.bench_function("hashmap single thread", |b| {
        b.iter(|| {
            crate::db::skip_list::bench::single_thread_bench(&mut hashMap_list, config, &mut r);
        });
    });

    group.finish();
}

criterion_group!(benches, bench_skipList_single_thread);
criterion_main!(benches);
