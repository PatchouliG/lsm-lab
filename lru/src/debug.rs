#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

mod debug {
    use std::fmt::Display;

    struct A<T> {
        a: T,
    }

    impl<T> Drop for A<T> {
        fn drop(&mut self) {
            println!("hi");
        }
    }

    impl<T: Display> A<T> {
        fn print(&self) -> &str {
            "dsf"
        }
    }

    #[cfg(test)]
    mod test {
        use serde::{Deserialize, Serialize};
        use serde_json::Result;
        use std::fmt::Write;
        use std::ops::Add;

        #[derive(Serialize, Deserialize)]
        struct Address {
            street: String,
            city: String,
        }

        fn print_an_address() -> Result<()> {
            // Some data structure.
            let address = Address {
                street: "10 Downing Street".to_owned(),
                city: "London".to_owned(),
            };

            // Serialize it to a JSON string.
            let j = serde_json::to_string(&address)?;

            // Print, write to a file, or send to an HTTP server.
            println!("{}", j);
            let a: Address = serde_json::from_str(&j)?;
            println!("{}", a.street);
        }
        use super::A;
        use std::borrow::Borrow;
        use std::cell::RefCell;
        use std::sync::{Arc, Mutex};
        use std::thread::spawn;

        #[test]
        fn test() {
            let a = Box::new(A { a: 3 });
            let b = Box::into_raw(a);
            unsafe {
                Box::from_raw(b);
            }
        }

        use log::{info, warn};
        use log::{LevelFilter, SetLoggerError};

        use simplelog::*;
        use std::fs::File;

        pub fn init() {
            WriteLogger::init(
                LevelFilter::Debug,
                Config::default(),
                File::create("test234").unwrap(),
            );

            // let res = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info));
            // res
        }
        #[test]
        fn test() {}
    }
}
