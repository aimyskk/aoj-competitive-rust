use std::io;

fn read_int2() -> (i64, i64) {
  let mut line = String::new();
  io::stdin().read_line(&mut line).ok();
  let mut iter = line.split_whitespace().map(|s| s.parse().unwrap());

  (iter.next().unwrap(), iter.next().unwrap())
}

fn main() {
  let (h,w) = read_int2();
  println!("{} {}", h*w, 2*(h+w));
}
