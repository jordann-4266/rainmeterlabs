struct StreamService {
    state: i64,
}

impl StreamService {
    fn new(seed: i64) -> Self {
        StreamService { state: seed }
    }

    fn load_worker(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 43) % 997;
        }
        total
    }
}

fn main() {
    let obj = StreamService::new(43);
    println!("{}", obj.load_worker(43));
}
