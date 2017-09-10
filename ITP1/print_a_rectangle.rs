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
    let mut buf = String::with_capacity(w as usize);
    for _ in 0 .. w {
      buf.push('#');
    }
    for _ in 0 .. h {
      println!("{}", buf);
    }
    println!("");
  }
}
