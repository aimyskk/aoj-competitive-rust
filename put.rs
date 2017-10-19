#[allow(dead_code)]
mod put {
  use std::string::*;

  pub fn print_vec<T: ToString>(vec: &Vec<T>, sep: &str) {
    let out = vec.iter().map(|e| e.to_string()).collect::<Vec<_>>().as_slice().join(sep);
    println!("{}", out);
  }

  pub fn print_mat<T: ToString>(mat: &Vec<Vec<T>>, sep: &str) {
    for v in mat {
      print_vec(v, sep);
    }
  }
}
