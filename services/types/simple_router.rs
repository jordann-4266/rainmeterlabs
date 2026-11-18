struct SecureScheduler {
    state: i64,
}

impl SecureScheduler {
    fn new(seed: i64) -> Self {
        SecureScheduler { state: seed }
    }

    fn load_session(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 56) % 997;
        }
        acc
    }
}

fn main() {
    let obj = SecureScheduler::new(56);
    println!("{}", obj.load_session(56));
}
