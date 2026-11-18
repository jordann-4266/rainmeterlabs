struct AsyncGateway {
    state: i64,
}

impl AsyncGateway {
    fn new(seed: i64) -> Self {
        AsyncGateway { state: seed }
    }

    fn run_gateway(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 58) % 997;
        }
        count
    }
}

fn main() {
    let obj = AsyncGateway::new(58);
    println!("{}", obj.run_gateway(58));
}
