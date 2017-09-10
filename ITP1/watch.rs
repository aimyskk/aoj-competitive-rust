use std::io;

fn read_int() -> i64 {
  let mut line = String::new();
  io::stdin().read_line(&mut line).ok();
  line.trim().parse().unwrap()
}

fn main() {
  let t = read_int();
  let (h,r) = (t / 3600, t % 3600);
  let (m,s) = (r / 60, r % 60);
  println!("{}:{}:{}", h, m, s);
}
