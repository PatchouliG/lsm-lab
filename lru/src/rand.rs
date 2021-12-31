pub mod simple_rand {
    const A: u64 = 16807;
    const C: u64 = 0;
    const M: u64 = i32::MAX as u64 - 1;



    pub struct SimpleRand {
        next: u64,
    }

    impl SimpleRand {
        pub fn new() -> SimpleRand {
            SimpleRand::with_seed(42)
        }
        pub fn with_seed(seed: u64) -> SimpleRand {
            let mut res = SimpleRand { next: seed };
            res.next();
            res
        }
        pub fn next(&mut self) -> u64 {
            let res = self.next;
            self.next = (res * A + C) % M;
            res
        }
        pub fn next_range(&mut self, range: u64) -> u64 {
            self.next() % range
        }
        pub fn next_percent(&mut self) -> f64 {
            ((self.next() % 100) as f64) / 100.0
        }
    }



    #[cfg(test)]
    mod test {
        #[test]
        fn test() {
            let mut r = super::SimpleRand::new();
            assert_eq!(r.next(), 705894);
            assert_eq!(r.next(), 1126542228);
            assert_eq!(r.next(), 1579402860);
            assert_eq!(r.next_percent(), 0.6);
            assert_eq!(r.next_range(100), 72);
        }
    }
}