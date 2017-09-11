fn read<T: std::str::FromStr>() -> T {
  let mut buf = String::new();
  std::io::stdin().read_line(&mut buf).ok();
  buf.trim().parse::<T>().ok().unwrap()
}

fn reads<T: std::str::FromStr>(n: u32) -> Vec<T> {
  let mut vec: Vec<T> = vec![];
  for _ in 0 .. n {
    vec.push(read());
  }
  vec
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
  let mut buf = String::new();
  std::io::stdin().read_line(&mut buf).ok();
  buf.trim().split_whitespace().map(|t| t.parse::<T>().ok().unwrap()).collect()
}

fn read_tuple<T1: std::str::FromStr, T2: std::str::FromStr>() -> (T1, T2) {
  let mut buf = String::new();
  std::io::stdin().read_line(&mut buf).ok();
  let mut it = buf.trim().split_whitespace();
  let x = it.next().unwrap().parse::<T1>().ok().unwrap();
  let y = it.next().unwrap().parse::<T2>().ok().unwrap();
  (x, y)
}

fn read_tuples<T1: std::str::FromStr, T2: std::str::FromStr>(n: u32) -> Vec<(T1, T2)> {
  let mut vec: Vec<(T1, T2)> = vec![];
  for _ in 0 .. n {
    vec.push(read_tuple());
  }
  vec
}
