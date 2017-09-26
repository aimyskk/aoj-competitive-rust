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

fn is_leap (y: u32) -> bool {
  y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)
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

fn permutations<T: Clone + std::fmt::Debug>(vec: &Vec<T>) -> Vec<Vec<T>> {
  fn go<T: Clone + std::fmt::Debug>(vec: &Vec<T>, n: usize, m: usize, k: usize, buf: &mut Vec<usize>, acc: &mut Vec<Vec<T>>) {
    if k == 0 {
      let r: Vec<T> = buf.iter().map(|&i| vec[i-1].clone()).collect();
      acc.push(r);
    } else {
      for x in n .. m + 1 {
        if !buf.contains(&x) {
          buf.push(x);
          go(vec, n, m, k-1, buf, acc);
          buf.pop();
        }
      }
    }
  }

  let mut acc = vec![];
  go(vec, 1, vec.len(), vec.len(), &mut vec![], &mut acc);
  acc
}
