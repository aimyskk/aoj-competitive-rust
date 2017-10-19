#[allow(dead_code)]
mod get {
  use std::io::*;
  use std::str::*;

  pub fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    buf.trim().parse::<T>().ok().unwrap()
  }

  pub fn reads<T: FromStr>(n: usize) -> Vec<T> {
    let mut vec: Vec<T> = vec![];
    for _ in 0 .. n {
      vec.push(read());
    }
    vec
  }

  pub fn read_tuple<T1: FromStr, T2: FromStr>() -> (T1, T2) {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    let mut it = buf.trim().split_whitespace();
    let x = it.next().unwrap().parse::<T1>().ok().unwrap();
    let y = it.next().unwrap().parse::<T2>().ok().unwrap();
    (x, y)
  }

  pub fn read_tuples<T1: FromStr, T2: FromStr>(n: usize) -> Vec<(T1, T2)> {
    let mut vec: Vec<(T1, T2)> = vec![];
    for _ in 0 .. n {
      vec.push(read_tuple());
    }
    vec
  }

  pub fn read_3tuple<T1: FromStr, T2: FromStr, T3: FromStr>() -> (T1, T2, T3) {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    let mut it = buf.trim().split_whitespace();
    let x = it.next().unwrap().parse::<T1>().ok().unwrap();
    let y = it.next().unwrap().parse::<T2>().ok().unwrap();
    let z = it.next().unwrap().parse::<T3>().ok().unwrap();
    (x, y, z)
  }

  pub fn read_3tuples<T1: FromStr, T2: FromStr, T3: FromStr>(n: usize) -> Vec<(T1, T2, T3)> {
    let mut vec: Vec<(T1, T2, T3)> = vec![];
    for _ in 0 .. n {
      vec.push(read_3tuple());
    }
    vec
  }

  pub fn read_vec<T: FromStr>() -> Vec<T> {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    buf.trim().split_whitespace().map(|t| t.parse::<T>().ok().unwrap()).collect()
  }

  pub fn read_mat<T: FromStr>(h: usize) -> Vec<Vec<T>> {
    let mut mat: Vec<Vec<T>> = vec![];
    for _ in 0 .. h {
      mat.push(read_vec());
    }
    mat
  }
}
