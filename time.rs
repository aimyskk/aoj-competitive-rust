use std::time::*;
struct Time {
  start: Instant
}

#[allow(dead_code)]
impl Time {
  fn new() -> Time {
    Time {start: Instant::now()}
  }

  fn elapsed(&self) -> f64 {
    let elapsed = self.start.elapsed();
    let sec = elapsed.as_secs() as f64;
    let nano = elapsed.subsec_nanos() as f64 / 1000000000.0;
    sec + nano
  }
}
