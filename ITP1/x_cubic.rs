use std::io;

fn read_int() -> i64 {
  let mut line = String::new();
  io::stdin().read_line(&mut line).ok();
  line.trim().parse().unwrap()
}

fn main() {
  let ans = read_int();
  println!("{}", ans.pow(3));
}
