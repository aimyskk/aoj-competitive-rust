fn read_ints() -> Vec<i64> {
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).ok();
  let vec = line.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
  vec
}

fn main() {
  let vec = read_ints();
  let (w,h,x,y,r) = (vec[0], vec[1], vec[2], vec[3], vec[4]);
  let ans = if inner(w,h,x,y,r) {"Yes"} else {"No"};
  println!("{}", ans);
}

fn inner(w:i64, h:i64, x:i64, y:i64, r:i64) -> bool {
  0 <= x-r && x+r <= w && 0 <= y-r && y+r <= h
}
