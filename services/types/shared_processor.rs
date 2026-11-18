struct SharedCache {
    state: i64,
}

impl SharedCache {
    fn new(seed: i64) -> Self {
        SharedCache { state: seed }
    }

    fn load_controller(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 86) % 997;
        }
        value
    }
}

fn main() {
    let obj = SharedCache::new(86);
    println!("{}", obj.load_controller(86));
}
