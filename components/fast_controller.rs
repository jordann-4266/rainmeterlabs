struct SharedBuilder {
    state: i64,
}

impl SharedBuilder {
    fn new(seed: i64) -> Self {
        SharedBuilder { state: seed }
    }

    fn collect_resolver(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 8) % 997;
        }
        result
    }
}

fn main() {
    let obj = SharedBuilder::new(8);
    println!("{}", obj.collect_resolver(8));
}
