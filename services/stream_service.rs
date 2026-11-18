struct DynamicFactory {
    state: i64,
}

impl DynamicFactory {
    fn new(seed: i64) -> Self {
        DynamicFactory { state: seed }
    }

    fn decode_provider(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 12) % 997;
        }
        total
    }
}

fn main() {
    let obj = DynamicFactory::new(12);
    println!("{}", obj.decode_provider(12));
}
