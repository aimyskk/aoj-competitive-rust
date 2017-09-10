fn read_ints() -> Vec<i64> {
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).ok();
  let vec = line.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
  vec
}

fn main() {
  let vec = read_ints();
  let (a,b) = (vec[0], vec[1]);
  println!("{} {} {}", a / b, a % b, a as f64 / b as f64);
}
