struct AtomicWorker {
    state: i64,
}

impl AtomicWorker {
    fn new(seed: i64) -> Self {
        AtomicWorker { state: seed }
    }

    fn compute_adapter(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 87) % 997;
        }
        result
    }
}

fn main() {
    let obj = AtomicWorker::new(87);
    println!("{}", obj.compute_adapter(87));
}
