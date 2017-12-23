struct Graph {
  size: usize,
  adj: Vec<Vec<(usize, isize)>>
}

impl Clone for Graph {
  fn clone(&self) -> Graph {
    Graph {
      size: self.size,
      adj: self.adj.clone()
    }
  }
}

#[allow(dead_code)]
impl Graph {
  fn new_nonlabeled(n: usize, edges: &[(usize, usize)]) -> Graph {
    let edges = edges.iter().map(|&(a,b)|(a,b,1)).collect::<Vec<_>>();
    Graph::new_labeled(n, &edges)
  }

  fn new_labeled(n: usize, edges: &[(usize, usize, isize)]) -> Graph {
    let mut g: Vec<Vec<(usize, isize)>> = vec![vec![]; n+1];
    for &(a, b, c) in edges {
      g[a].push((b,c));
      g[b].push((a,c));  // delete for digraph
    }
    Graph {size: n, adj: g}
  }

  fn edges(&self) -> Vec<(usize, usize, isize)> {
    let mut buf = vec![];
    for (i, next) in self.adj.iter().enumerate() {
      for &(j, x) in next {
        buf.push((i, j, x))
      }
    }
    buf
  }

  fn is_adjacent(&self, u: usize, v: usize) -> bool {
    self.adj[u].iter().any(|&(w,_)|w==v) || self.adj[v].iter().any(|&(w,_)|w==u)
  }

  fn is_connected(&self) -> bool {
    self.dfs(1).len() == self.size
  }

  fn is_hitofude(&self) -> bool {
    let deg_odd = self.adj.iter().filter(|vs|vs.len()%2==1).count();
    deg_odd == 0 || deg_odd == 2
  }

  fn bellman_ford(&self, s: usize) -> Vec<isize> {
    const INF: isize = std::isize::MAX >> 1;
    let edges = self.edges();
    let mut bf: Vec<isize> = vec![INF; self.size];
    bf[s] = 0;

    for _ in 1 .. self.size {
      for &(v, w, c) in &edges {
        bf[w] = std::cmp::min(bf[w], bf[v]+c)
      }
    }
    bf
  }

  fn dijkstra(&self, s: usize) -> Vec<isize> {
    use std::collections::BinaryHeap;

    const INF: isize = std::isize::MAX;
    let mut dk: Vec<isize> = vec![INF; self.size];
    dk[s] = 0;

    let mut pq = BinaryHeap::new();
    pq.push((0,s));

    while let Some((acc,v)) = pq.pop() {
      let acc = -acc;
      for &(w,c) in &self.adj[v] {
        let cand = acc + c;
        if cand < dk[w] {
          dk[w] = cand;
          pq.push((-cand,w));
        }
      }
    }
    dk
  }

  fn warshall_floyd(&self) -> Vec<Vec<isize>> {
    const INF: isize = std::isize::MAX >> 1;
    let mut wf: Vec<Vec<isize>> = vec![vec![INF; self.size+1]; self.size+1];
    for i in 1 .. self.size+1 {wf[i][i] = 0}

    for (next, i) in self.adj.iter().zip(0..) {
      for &(j, x) in next {
        wf[i][j] = x
      }
    }
    for k in 1 .. self.size+1 {
      for i in 1 .. self.size+1 {
        for j in 1 .. self.size+1 {
          wf[i][j] = std::cmp::min(wf[i][j], wf[i][k] + wf[k][j]);
        }
      }
    }
    wf
  }

  fn coloring2(&self) -> Option<(Vec<usize>, Vec<usize>)> {
    fn paint(v: usize, p: bool, g: &Graph, cv: &mut Vec<Option<bool>>) -> bool {
      match cv[v] {
        None => {
          let next = &g.adj[v];
          cv[v] = Some(p);
          next.iter().all(|&(w,_)|paint(w,!p,g,cv))},
        Some(q) => {q == p}
      }
    }

    let mut canvas: Vec<Option<bool>> = vec![None; self.size+1];
    let ans = paint(1, false, self, &mut canvas);
    let bs = canvas.iter().enumerate().filter(|&(_,&v)|v==Some(false)).map(|(i,_)|i).collect::<Vec<_>>();
    let ws = canvas.iter().enumerate().filter(|&(_,&v)|v==Some(true)).map(|(i,_)|i).collect::<Vec<_>>();
    if ans {Some((bs,ws))} else {None}
  }

  fn dfs(&self, s: usize) -> Vec<usize> {
    fn go(g: &Graph, current: usize, mut path: &mut Vec<usize>, mut visited: &mut Vec<bool>) {
      for &(next, _) in &g.adj[current] {
        if visited[next] {
          continue
        } else {
          visited[next] = true;
          path.push(next);
          go(&g, next, &mut path, &mut visited)
        }
      }
    }

    let mut path = vec![s];
    let mut visited = vec![false; self.size+1];
    visited[s] = true;
    go(&self, s, &mut path, &mut visited);
    path
  }

  fn bfs(&self, v: usize) -> Vec<usize> {
    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    let mut buf = vec![];
    let mut vd = vec![false; self.size+1];
    vd[v] = true;
    q.push_back(v);
    while let Some(w) = q.pop_front() {
      buf.push(w);
      for &(next,_) in self.adj[w].iter() {
        if !vd[next] {
          q.push_back(next);
          vd[next] = true
        }
      }
    }
    buf
  }

  fn ultimate(&self, v: usize) -> usize {
    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    let mut dist = 0;
    let mut vd = vec![false; self.size+1];
    vd[v] = true;
    q.push_back((v,dist));
    while let Some((w, d)) = q.pop_front() {
      dist = std::cmp::max(dist, d);
      for &(next,_) in self.adj[w].iter() {
        if !vd[next] {
          q.push_back((next, d+1));
          vd[next] = true
        }
      }
    }
    dist
  }

  fn cut(&mut self, v: usize, w: usize) {
    self.adj[v].retain(|&(t,_)| t != w);
    self.adj[w].retain(|&(t,_)| t != v);
  }
}
