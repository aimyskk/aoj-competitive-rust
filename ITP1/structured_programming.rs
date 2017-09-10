use std::io::*;

fn read_int() -> i64 {
  let mut buf = String::new();
  stdin().read_line(&mut buf).ok();
  buf.trim().parse().unwrap()
}

fn main() {
  let n = read_int();
  for x in (3 .. n+1).filter(|x| x % 3 == 0 || x.to_string().chars().any(|c| c=='3')) {
    print!(" {}", x);
  }
  println!("");
}
