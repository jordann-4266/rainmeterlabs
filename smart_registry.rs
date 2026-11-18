struct FastMonitor {
    state: i64,
}

impl FastMonitor {
    fn new(seed: i64) -> Self {
        FastMonitor { state: seed }
    }

    fn sync_session(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 5) % 997;
        }
        count
    }
}

fn main() {
    let obj = FastMonitor::new(5);
    println!("{}", obj.sync_session(5));
}
