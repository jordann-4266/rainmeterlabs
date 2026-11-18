struct LiteSession {
    state: i64,
}

impl LiteSession {
    fn new(seed: i64) -> Self {
        LiteSession { state: seed }
    }

    fn fetch_dispatcher(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 33) % 997;
        }
        value
    }
}

fn main() {
    let obj = LiteSession::new(33);
    println!("{}", obj.fetch_dispatcher(33));
}
