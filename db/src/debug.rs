#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod debug_tmp {
    use std::collections::hash_map::RandomState;
    use std::fmt::Display;

    trait Test<A> {
        fn foo(&self, a: A) -> A;
    }

    struct B {
        a: i32,
    }

    impl Test<i32> for TB {
        fn foo(&self, a: i32) -> i32 {
            a
        }
    }
    impl Test<i32> for B {
        fn foo(&self, a: i32) -> i32 {
            todo!()
        }
    }

    struct TB {
        s: Vec<u8>,
    }

    impl TB {
        fn new() -> TB {
            TB { s: Vec::new() }
        }
        fn append(&mut self, data: &[u8]) {}
    }

    // &self.i
    trait F<A> {}

    struct TC<'a> {
        a: &'a i32,
    }

    impl<'a> TC<'a> {
        fn new(m: &'a i32) -> TC<'a> {
            TC { a: m }
        }
    }

    impl Test<u32> for TB {
        fn foo(&self, a: u32) -> u32 {
            todo!()
        }
    }

    struct A<T> {
        a: T,
    }

    impl<T> Drop for A<T> {
        fn drop(&mut self) {
            println!("hi");
        }
    }

    #[cfg(test)]
    mod test {
        use std::borrow::Borrow;
        use std::cell::RefCell;
        use std::ptr::slice_from_raw_parts_mut;
        use std::sync::atomic::{AtomicI8, AtomicPtr, Ordering};
        use std::sync::{Arc, Mutex};
        use std::thread::spawn;

        use crate::debug::debug_tmp::{Test, TB};

        use super::A;
        fn test_test<T>(t: &dyn Test<T>, a: T) {
            t.foo(a);
        }

        #[test]
        fn test() {
            let i = 32;
            let tb = TB::new();
            test_test(&tb, i);
        }
    }
}
