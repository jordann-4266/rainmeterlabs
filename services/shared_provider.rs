struct FastSession {
    state: i64,
}

impl FastSession {
    fn new(seed: i64) -> Self {
        FastSession { state: seed }
    }

    fn resolve_processor(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 48) % 997;
        }
        value
    }
}

fn main() {
    let obj = FastSession::new(48);
    println!("{}", obj.resolve_processor(48));
}
