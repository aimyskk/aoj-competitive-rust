use std::io;

fn read_int3() -> (i64, i64, i64) {
  let mut line = String::new();
  io::stdin().read_line(&mut line).ok();
  let mut iter = line.split_whitespace().map(|s| s.parse().unwrap());

  (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
}

fn main() {
  let (a,b,c) = read_int3();
  let ans = if a<b && b<c {"Yes"} else {"No"};
  println!("{}", ans);
}
