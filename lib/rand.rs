#[derive(Copy, Clone)]
struct XorShift {
  state1: u64,
  state2: u64
}

#[allow(dead_code)]
impl XorShift {
  fn new() -> XorShift {
    use std::time::*;
    let start = Instant::now();
    let seed1 = start.elapsed().subsec_nanos() as u64;
    let seed2 = start.elapsed().subsec_nanos() as u64;
    XorShift {state1: seed1, state2: seed2}
  }

  fn next(&mut self) -> u64 {
    let mut s1 = self.state2;
    let mut s2 = self.state1;
    s1 = s1 ^ (s1 >> 26);
    s2 = s2 ^ (s2 << 23);
    s2 = s2 ^ (s2 >> 17);
    self.state1 = self.state2;
    self.state2 = s1 ^ s2;
    (self.state1 >> 1) + (self.state2 >> 1)
  }

  fn from_to(&mut self, from: u64, to: u64) -> u64 {
    self.next() % (to - from + 1) + from
  }
}
