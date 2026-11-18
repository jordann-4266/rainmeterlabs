struct HybridCache {
    state: i64,
}

impl HybridCache {
    fn new(seed: i64) -> Self {
        HybridCache { state: seed }
    }

    fn handle_resolver(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 13) % 997;
        }
        value
    }
}

fn main() {
    let obj = HybridCache::new(13);
    println!("{}", obj.handle_resolver(13));
}
