use std::io::*;

fn read_triple(line: &str) -> (i64, String, i64) {
  let mut iter = line.split_whitespace();

  let a = iter.next().unwrap().parse().unwrap();
  let op = iter.next().unwrap().to_string();
  let b = iter.next().unwrap().parse().unwrap();

  (a, op, b)
}

fn read_exps() -> Vec<(i64, String, i64)> {
  let mut buf = String::new();
  stdin().read_to_string(&mut buf).ok();
  let mut vec: Vec<_> = buf.trim().lines().map(|t| read_triple(&t)).collect();
  vec.pop();
  vec
}

fn main() {
  let vec = read_exps();
  for (a,op,b) in vec {
    let ans = if op == "+" {a + b}
      else if op == "-" {a - b}
      else if op == "*" {a * b}
      else if op == "/" {a / b}
      else {0};
    println!("{}", ans);
  }
}
