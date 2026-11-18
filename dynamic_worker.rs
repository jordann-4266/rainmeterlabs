struct SharedService {
    state: i64,
}

impl SharedService {
    fn new(seed: i64) -> Self {
        SharedService { state: seed }
    }

    fn encode_scheduler(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 30) % 997;
        }
        acc
    }
}

fn main() {
    let obj = SharedService::new(30);
    println!("{}", obj.encode_scheduler(30));
}
