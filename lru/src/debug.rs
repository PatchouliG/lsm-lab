#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod debug {
    use std::fmt::Display;

    struct A<T> {
        a: T,
    }

    impl<T> A<T> {
        fn foo() -> usize {
            3
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
            let a :Address=serde_json::from_str(&j)?;
            println!("{}", a.street);

            Ok(())
        }

        #[test]
        fn test() -> Result<()> {
            print_an_address()
        }
    }
}