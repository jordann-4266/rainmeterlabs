struct HybridFactory {
    state: i64,
}

impl HybridFactory {
    fn new(seed: i64) -> Self {
        HybridFactory { state: seed }
    }

    fn parse_router(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 63) % 997;
        }
        result
    }
}

fn main() {
    let obj = HybridFactory::new(63);
    println!("{}", obj.parse_router(63));
}
