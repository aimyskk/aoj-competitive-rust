use std::io::*;

fn read_int() -> i64 {
  let mut buf = String::new();
  stdin().read_line(&mut buf).ok();
  buf.trim().parse().unwrap()
}

fn read_ints() -> Vec<i64> {
  let mut buf = String::new();
  stdin().read_line(&mut buf).ok();
  buf.trim().split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn main() {
  let _ = read_int();
  let vec = read_ints();

  let min = &vec.iter().min().unwrap();
  let max = &vec.iter().max().unwrap();
  let sum: i64 = vec.iter().sum();

  println!("{} {} {}", min, max, sum);
}
