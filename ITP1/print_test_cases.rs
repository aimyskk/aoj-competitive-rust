use std::io::Read;

fn read_ints() -> Vec<i64> {
  let mut line = String::new();
  std::io::stdin().read_to_string(&mut line).ok();
  let vec = line.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
  vec
}

fn main() {
  let mut vec = read_ints();
  vec.pop();
  for (i, x) in vec.into_iter().enumerate() {
    println!("Case {}: {}", i+1, x);
  }
}
