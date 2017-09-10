use std::io::*;

fn read_int_pairs() -> Vec<(i64,i64)> {
  let mut buf = String::new();
  stdin().read_to_string(&mut buf).ok();
  buf.trim().lines().map(|s| make_pair(&s)).collect()
}

fn make_pair(line: &str) -> (i64, i64) {
  let mut iter = line.split_whitespace().map(|s| s.parse().unwrap());
  (iter.next().unwrap(), iter.next().unwrap())
}

fn main() {
  let vec = read_int_pairs();
  for (h,w) in vec.into_iter().take_while(|x| *x != (0,0)) {
    for i in 0 .. h {
      for j in 0 .. w {
        print!("{}", if (i+j)%2 == 0 {"#"} else {"."});
      }
      println!("");
    }
    println!("");
  }
}
