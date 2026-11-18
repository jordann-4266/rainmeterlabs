struct SharedGateway {
    state: i64,
}

impl SharedGateway {
    fn new(seed: i64) -> Self {
        SharedGateway { state: seed }
    }

    fn sync_registry(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 68) % 997;
        }
        value
    }
}

fn main() {
    let obj = SharedGateway::new(68);
    println!("{}", obj.sync_registry(68));
}
