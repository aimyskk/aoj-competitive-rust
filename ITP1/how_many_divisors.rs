fn read_ints() -> Vec<i64> {
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).ok();
  let vec = line.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
  vec
}

fn main() {
  let vec = read_ints();
  let (a,b,c) = (vec[0], vec[1], vec[2]);
  println!("{}", num_of_divisors(a,b,c));
}

fn num_of_divisors (a:i64, b:i64, c:i64) -> usize {
  (a .. b+1).filter(|x| c % x == 0).count()
}
