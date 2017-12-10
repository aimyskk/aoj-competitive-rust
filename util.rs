fn lcm(x: usize, y: usize) -> usize {
  (x * y) / gcd(x, y)
}

fn gcd(a: usize, b: usize) -> usize {
  if b == 0 {a} else {gcd(b, a % b)}
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

fn group_by<T: PartialEq + Clone, F>(vec: &Vec<T>, f: F) -> Vec<Vec<T>> where F: Fn(&T, &T) -> bool {
  let l = vec.len();
  let mut gs = vec![];
  let mut g = vec![];
  for i in 0 .. l {
    g.push(vec[i].clone());
    if i < l-1 && !f(&vec[i], &vec[i+1]) {
      gs.push(g.clone());
      g.clear()
    }
  }
  gs.push(g.clone());
  gs
}
