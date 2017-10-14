struct UnionFind {
  rp: Vec<usize>,
  rk: Vec<usize>
}

impl UnionFind {
  fn new(n: usize) -> UnionFind {
    UnionFind {
      n: n,
      rp: (0..n).collect(),
      rk: vec![1; n]
    }
  }

  fn rep(&self, p: usize) -> usize {
    if self.rp[p] == p {p} else {self.rep(self.rp[p])}
  }

  fn rank(&self, p: usize) -> usize {
    self.rk[self.rep(p)]
  }

  fn same(&self, p: usize, q: usize) -> bool {
    self.rep(p) == self.rep(q)
  }

  fn count(&self) -> usize {
    let mut reps = (0..self.n).map(|x|self.rep(x)).collect::<Vec<_>>();
    reps.sort();
    reps.dedup();
    reps.len()
  }

  fn unite(&mut self, p: usize, q:usize) {
    let rkp = self.rank(p);
    let rkq = self.rank(q);
    let (p, q) = if rkp <= rkq {(p, q)} else {(q, p)};

    let repp = self.rep(p);
    let repq = self.rep(q);
    self.rp[p] = repq;
    self.rp[repp] = repq;
    self.rk[repq] += self.rk[repp];
  }
}
