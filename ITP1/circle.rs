use std::io::*;
use std::f64::consts::*;

fn read_real() -> f64 {
  let mut buf = String::new();
  stdin().read_line(&mut buf).ok();
  buf.trim().parse().unwrap()
}

fn main() {
  let r = read_real();
  println!("{} {}", PI * r.powi(2), 2.0 * PI * r);
}
