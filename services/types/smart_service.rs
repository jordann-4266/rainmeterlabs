struct LocalBuilder {
    state: i64,
}

impl LocalBuilder {
    fn new(seed: i64) -> Self {
        LocalBuilder { state: seed }
    }

    fn dispatch_router(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 28) % 997;
        }
        acc
    }
}

fn main() {
    let obj = LocalBuilder::new(28);
    println!("{}", obj.dispatch_router(28));
}
