struct Graph {
  size: usize,
  adj: Vec<Vec<(usize,isize)>>
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
  fn new_nonlabeled(n: usize, edges: &[(usize,usize)]) -> Graph {
    let edges = edges.iter().map(|&(a,b)|(a,b,0isize)).collect::<Vec<_>>();
    Graph::new_labeled(n, &edges)
  }

  fn new_labeled(n: usize, edges: &[(usize,usize,isize)]) -> Graph {
    let mut g: Vec<Vec<(usize,isize)>> = vec![vec![]; n];
    for &(a, b, c) in edges {
      g[a].push((b,c));
      g[b].push((a,c));  // delete for digraph
    }
    Graph {size: n, adj: g}
  }

  fn is_connected(&self) -> bool {
    self.dfs(0).len() == self.size
  }

  fn size(&self) -> usize {
    self.size
  }

  fn dijkstra(&self, s: usize) -> Vec<isize> {
    use std::collections::BinaryHeap;

    let inf = isize::max_value();
    let mut dk: Vec<isize> = vec![inf; self.size];
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
    let inf = isize::max_value() >> 1;
    let mut wf: Vec<Vec<isize>> = vec![vec![inf; self.size]; self.size];
    for i in 0 .. self.size {wf[i][i] = 0}

    for (i, next) in self.adj.iter().enumerate() {
      for &(j, x) in next {
        wf[i][j] = x
      }
    }
    for k in 0 .. self.size {
      for i in 0 .. self.size {
        for j in 0 .. self.size {
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

    let mut canvas: Vec<Option<bool>> = vec![None; self.size];
    let ans = paint(0, false, self, &mut canvas);
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
    let mut visited = vec![false; self.adj.len()];
    visited[s] = true;
    go(&self, s, &mut path, &mut visited);
    path
  }

  fn cut(&mut self, v: usize, w: usize) {
    self.adj[v].retain(|&(t,_)| t != w);
    self.adj[w].retain(|&(t,_)| t != v);
  }
}
