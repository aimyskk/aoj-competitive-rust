fn read_ints() -> Vec<i64> {
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).ok();
  let vec = line.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
  vec
}

fn main() {
  let mut vec = read_ints();
  vec.sort();
  let ans = vec.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ");
  println!("{}", ans);
}
