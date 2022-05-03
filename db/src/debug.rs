#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod debug_tmp {
    use std::collections::hash_map::RandomState;
    use std::fmt::Display;

    trait Test<A> {
        type B;
        fn foo(&self, a: A) -> B;
    }

    #[derive(Copy)]
    struct B<'a> {
        a: &'a i32,
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

    impl Test<i32> for TB {
        type B = i32;

        fn foo(&self, a: i32) -> B {
            todo!()
        }
    }

    impl Test<u32> for TB {
        type B = i32;

        fn foo(&self, a: u32) -> B {
            todo!()
        }
    }

    impl<'a> Clone for B<'a> {
        fn clone(&self) -> Self {
            todo!()
            // B { a: self.a }
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

        use crate::debug::debug_tmp::TB;

        use super::A;

        #[test]
        fn test() {
            let a = format!("{}:{}", 3, 4);
            println!("{}", a);
            let mut b = TB::new();
            let c: Vec<u8> = vec![1, 23, 54];
            b.append(c.as_ref());
        }
    }
}
