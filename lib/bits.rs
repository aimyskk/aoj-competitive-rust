trait Bits {
  fn set_bit(&mut self, d: usize);
  fn clear_bit(&mut self, d: usize);
  fn toggle_bit(&mut self, d: usize);
  fn complement(&mut self);
  fn test_bit(&self, d: usize) -> bool;
  fn popcount(&self) -> usize;
}

#[allow(dead_code)]
impl Bits for usize {
  fn set_bit(&mut self, d: usize) {
    *self |= 1 << d
  }

  fn clear_bit(&mut self, d: usize) {
    *self &= !(1 << d)
  }

  fn toggle_bit(&mut self, d: usize) {
    *self ^= 1 << d
  }

  fn complement(&mut self) {
    *self = !*self
  }

  fn test_bit(&self, d: usize) -> bool {
    self & (1 << d) != 0
  }

  fn popcount(&self) -> usize {
    self.count_ones() as usize
  }
}
