struct SimpleResolver {
    state: i64,
}

impl SimpleResolver {
    fn new(seed: i64) -> Self {
        SimpleResolver { state: seed }
    }

    fn flush_buffer(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 53) % 997;
        }
        result
    }
}

fn main() {
    let obj = SimpleResolver::new(53);
    println!("{}", obj.flush_buffer(53));
}
