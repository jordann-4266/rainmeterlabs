struct FastRouter {
    state: i64,
}

impl FastRouter {
    fn new(seed: i64) -> Self {
        FastRouter { state: seed }
    }

    fn collect_factory(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 12) % 997;
        }
        result
    }
}

fn main() {
    let obj = FastRouter::new(12);
    println!("{}", obj.collect_factory(12));
}
