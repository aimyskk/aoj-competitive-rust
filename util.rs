fn lcm(x: u32, y: u32) -> u32 {
  (x * y) / gcd(x, y)
}
 
fn gcd(x: u32, y: u32) -> u32 {
  let mut a = x;
  let mut b = y;
  let mut c;
  while a != 0 && b != 0 {
    c = b;
    b = a % b;
    a = c;
  }
  std::cmp::max(a, b)
}

fn rotate_mat<T: std::clone::Clone>(mat: &Vec<Vec<T>>) -> Vec<Vec<T>> {
  let h = mat.len();
  let w = mat[0].len();

  let mut rot: Vec<Vec<T>> = Vec::new();
  for j in 0 .. w {
    let mut vec: Vec<T> = Vec::new();
    for i in (0 .. h).rev() {
      vec.push(mat[i][j].clone());
    }
    rot.push(vec);
  }

  rot
}
