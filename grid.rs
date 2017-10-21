#[allow(dead_code)]
struct Grid<T: Clone + Copy + std::fmt::Debug> {
  height: usize,
  width: usize,
  mat: Vec<Vec<T>>
}

#[allow(dead_code)]
impl <T: Clone + Copy + std::fmt::Debug> Grid<T> {
  fn new(mat: Vec<Vec<T>>) -> Grid<T> {
    Grid {
      height: mat.len(),
      width: mat[0].len(),
      mat: mat
    }
  }

  fn height(&self) -> usize {
    self.height
  }

  fn width(&self) -> usize {
    self.width
  }

  fn get(&self, i: usize, j: usize) -> T {
    self.mat[i][j]
  }

  fn put(&mut self, i: usize, j: usize, x: T) {
    self.mat[i][j] = x
  }

  fn wall(&mut self, b: T) {
    for mut v in self.mat.iter_mut() {
      v.insert(0, b);
      v.push(b)
    }
    let ws = (0..self.width+2).map(|_|b).collect::<Vec<_>>();
    self.mat.insert(0, ws.clone());
    self.mat.push(ws);
  }

  fn print(&self) {
    for ref v in &self.mat {
      let out = v.iter().map(|e|format!("{:?}",e)).collect::<Vec<_>>().as_slice().join(" ");
      println!("{}", out);
    }
  }
}
