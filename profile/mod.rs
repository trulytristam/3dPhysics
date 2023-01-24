pub struct Profile {
    start: std::time::Instant,
    msg: &'static str,
}

impl Profile {
    pub fn new(msg: &'static str) -> Self {
        Self {
            start: std::time::Instant::now(),
            msg,
        }
    }
    pub fn evaluate(&self) {
        let time = (std::time::Instant::now() - self.start).as_nanos() as f64 / 1000000000.;
        println!("{}: {:?}", self.msg, time);
    }
}
