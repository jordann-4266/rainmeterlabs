struct RemoteSession {
    state: i64,
}

impl RemoteSession {
    fn new(seed: i64) -> Self {
        RemoteSession { state: seed }
    }

    fn encode_session(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 46) % 997;
        }
        result
    }
}

fn main() {
    let obj = RemoteSession::new(46);
    println!("{}", obj.encode_session(46));
}
