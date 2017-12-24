#![allow(unused_imports, unused_macros, unknown_lints)]

use std::collections::*;
use std::f64::*;
use std::cmp::*;
use std::cmp::Ordering::*;

macro_rules! dump {($($e:expr), +) => {println!(concat!($(stringify!($e), " = {:?}\n"), +), $($e), +)}}
macro_rules! max {($e0:expr, $($e:expr), +) => {{let mut r = $e0; $(r = max(r, $e);)+ r}}}
macro_rules! min {($e0:expr, $($e:expr), +) => {{let mut r = $e0; $(r = min(r, $e);)+ r}}}
macro_rules! freq {($v:expr) => {{let mut h = HashMap::new(); for x in $v {*h.entry(x).or_insert(0) += 1} h}}}
macro_rules! foreach {($v:expr, $f:expr) => {for x in $v.iter_mut() {*x = $f(*x)}}}

fn main() {
  let
}

#[allow(dead_code)]
mod get {
  use std::io::*;
  use std::str::*;

  pub fn val<T: FromStr>() -> T {
    let mut buf = String::new();
    let s = stdin();
    s.lock().read_line(&mut buf).ok();
    buf.trim_right().parse::<T>().ok().unwrap()
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
    let s = stdin();
    s.lock().read_line(&mut buf).ok();
    let mut it = buf.trim_right().split_whitespace();
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
    let s = stdin();
    s.lock().read_line(&mut buf).ok();
    let mut it = buf.trim_right().split_whitespace();
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

  pub fn vec<T: FromStr>() -> Vec<T> {
    let mut buf = String::new();
    let s = stdin();
    s.lock().read_line(&mut buf).ok();
    buf.trim_right().split_whitespace().map(|t| t.parse::<T>().ok().unwrap()).collect()
  }

  pub fn mat<T: FromStr>(h: usize) -> Vec<Vec<T>> {
    let mut mat = vec![];
    for _ in 0 .. h {
      mat.push(vec());
    }
    mat
  }

  pub fn chars() -> Vec<char> {
    let mut buf = String::new();
    let s = stdin();
    s.lock().read_line(&mut buf).ok();
    buf.trim_right().chars().collect()
  }
}

#[allow(dead_code)]
mod put {
  use std::string::*;

  pub fn vec<T: ToString>(vec: &Vec<T>, sep: &str) {
    let out = vec.iter().map(|e| e.to_string()).collect::<Vec<_>>().as_slice().join(sep);
    println!("{}", out);
  }

  pub fn mat<T: ToString>(mat: &Vec<Vec<T>>, sep: &str) {
    for v in mat {
      vec(v, sep);
    }
  }
}
