#[allow(dead_code)]
mod get {
  use std::io::*;
  use std::str::*;

  pub fn val<T: FromStr>() -> T {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    buf.trim().parse::<T>().ok().unwrap()
  }

  pub fn vals<T: FromStr>(n: usize) -> Vec<T> {
    let mut vec: Vec<T> = vec![];
    for _ in 0 .. n {
      vec.push(val());
    }
    vec
  }

  pub fn tuple<T1: FromStr, T2: FromStr>() -> (T1, T2) {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    let mut it = buf.trim().split_whitespace();
    let x = it.next().unwrap().parse::<T1>().ok().unwrap();
    let y = it.next().unwrap().parse::<T2>().ok().unwrap();
    (x, y)
  }

  pub fn tuples<T1: FromStr, T2: FromStr>(n: usize) -> Vec<(T1, T2)> {
    let mut vec: Vec<(T1, T2)> = vec![];
    for _ in 0 .. n {
      vec.push(tuple());
    }
    vec
  }

  pub fn tuple3<T1: FromStr, T2: FromStr, T3: FromStr>() -> (T1, T2, T3) {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    let mut it = buf.trim().split_whitespace();
    let x = it.next().unwrap().parse::<T1>().ok().unwrap();
    let y = it.next().unwrap().parse::<T2>().ok().unwrap();
    let z = it.next().unwrap().parse::<T3>().ok().unwrap();
    (x, y, z)
  }

  pub fn tuple3s<T1: FromStr, T2: FromStr, T3: FromStr>(n: usize) -> Vec<(T1, T2, T3)> {
    let mut vec: Vec<(T1, T2, T3)> = vec![];
    for _ in 0 .. n {
      vec.push(tuple3());
    }
    vec
  }

  pub fn list<T: FromStr>() -> Vec<T> {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    buf.trim().split_whitespace().map(|t| t.parse::<T>().ok().unwrap()).collect()
  }

  pub fn lists<T: FromStr>(h: usize) -> Vec<Vec<T>> {
    let mut mat: Vec<Vec<T>> = vec![];
    for _ in 0 .. h {
      mat.push(list());
    }
    mat
  }
}
