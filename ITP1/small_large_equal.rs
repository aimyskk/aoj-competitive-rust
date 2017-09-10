use std::io;
use std::cmp::Ordering;

fn read_int2() -> (i64, i64) {
  let mut line = String::new();
  io::stdin().read_line(&mut line).ok();
  let mut iter = line.split_whitespace().map(|s| s.parse().unwrap());

  (iter.next().unwrap(), iter.next().unwrap())
}

fn main() {
  let (a,b) = read_int2();
  let op = match a.cmp(&b) {Ordering::Less => "<", Ordering::Greater => ">", Ordering::Equal => "=="};
  println!("a {} b", op);
}
