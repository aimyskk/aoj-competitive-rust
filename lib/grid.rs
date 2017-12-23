struct Grid {
  height: usize,
  width: usize,
  mat: Vec<Vec<isize>>
}

#[allow(dead_code)]
impl Grid {
  fn new(mat: Vec<Vec<isize>>, b: isize) -> Grid {
    let h = mat.len();
    let w = mat[0].len();
    let mut m = mat;

    for mut v in m.iter_mut() {
      v.insert(0, b);
      v.push(b)
    }
    let bs = (0..w+2).map(|_|b).collect::<Vec<_>>();
    m.insert(0, bs.clone());
    m.push(bs);

    Grid {height: h, width: w, mat: m}
  }
 
  fn get(&self, i: usize, j: usize) -> isize {
    self.mat[i][j]
  }
 
  fn put(&mut self, i: usize, j: usize, x: isize) {
    self.mat[i][j] = x
  }
 
  fn accumulate(&mut self) {
    for i in 1 .. self.height + 1 {
      for j in 1 .. self.width + 1 {
        self.mat[i][j] += self.mat[i][j-1]
      }
    }
    for i in 1 .. self.height + 1 {
      for j in 1 .. self.width + 1 {
        self.mat[i][j] += self.mat[i-1][j]
      }
    }
  }
 
  fn sum(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> isize {
    self.mat[x2][y2] - self.mat[x1-1][y2] - self.mat[x2][y1-1] + self.mat[x1-1][y1-1]
  }
 
  fn print(&self) {
    for v in self.mat[1 .. self.height+1].iter() {
      let out = &v[1..self.width+1].iter().map(|e|format!("{:?}",e)).collect::<Vec<_>>().as_slice().join(" ");
      println!("{}", out);
    }
  }
}
