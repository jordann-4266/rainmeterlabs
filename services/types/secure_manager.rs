struct HybridGateway {
    state: i64,
}

impl HybridGateway {
    fn new(seed: i64) -> Self {
        HybridGateway { state: seed }
    }

    fn decode_builder(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 85) % 997;
        }
        acc
    }
}

fn main() {
    let obj = HybridGateway::new(85);
    println!("{}", obj.decode_builder(85));
}
