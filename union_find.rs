#[allow(dead_code)]
struct UnionFind {
  size: usize,
  rp: Vec<usize>,
  rk: Vec<usize>
}

#[allow(dead_code)]
impl UnionFind {
  fn new(n: usize) -> UnionFind {
    UnionFind {
      size: n,
      rp: (0..n).collect(),
      rk: vec![1; n]
    }
  }

  fn rep(&self, p: usize) -> usize {
    if self.rp[p] == p {p} else {self.rp[self.rp[p]]}
  }

  fn rank(&self, p: usize) -> usize {
    self.rk[self.rep(p)]
  }

  fn same(&self, p: usize, q: usize) -> bool {
    self.rep(p) == self.rep(q)
  }

  fn count(&self) -> usize {
    let mut reps = (0..self.size).map(|x|self.rep(x)).collect::<Vec<_>>();
    reps.sort();
    reps.dedup();
    reps.len()
  }

  fn unite(&mut self, p: usize, q:usize) {
    let repp = self.rep(p);
    let repq = self.rep(q);
    if repp == repq {return}

    let rkp = self.rk[repp];
    let rkq = self.rk[repq];
    let (p, repp, repq) = if rkp <= rkq {(p, repp, repq)} else {(q, repq, repp)};

    self.rp[p] = repq;
    self.rp[repp] = repq;
    self.rk[repq] += self.rk[repp];
  }
}
