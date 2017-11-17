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
