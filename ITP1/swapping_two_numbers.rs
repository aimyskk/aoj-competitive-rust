use std::io::Read;
use std::cmp;

fn read_int_pairs() -> Vec<(i64,i64)> {
  let mut contents = String::new();
  std::io::stdin().read_to_string(&mut contents).ok();
  let mut vec: Vec<_> = contents.trim().lines().map(|s| make_pair(&s)).collect();
  vec.pop();
  vec
}

fn make_pair(line: &str) -> (i64, i64) {
  let mut iter = line.split_whitespace().map(|s| s.parse().unwrap());
  (iter.next().unwrap(), iter.next().unwrap())
}

fn main() {
  let vec = read_int_pairs();
  for (x,y) in vec {
    println!("{} {}", cmp::min(x,y), cmp::max(x,y));
  }
}
