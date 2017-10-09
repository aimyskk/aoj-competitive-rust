fn is_bipartite(g: &Vec<Vec<usize>>) -> (bool, Option<(usize, usize)>) {
  fn dfs(v: usize, p: bool, g: &Vec<Vec<usize>>, cv: &mut Vec<Option<bool>>) -> bool {
    match cv[v] {
      None => {
        let next = &g[v];
        cv[v] = Some(p);
        next.iter().all(|&w|dfs(w,!p,g,cv))},
      Some(q) => {q == p},
      _ => unreachable!()
    }
  }

  let mut canvas: Vec<Option<bool>> = vec![None; g.len()];
  let ans = dfs(1, false, g, &mut canvas);
  let b = canvas.iter().filter(|&&v|v==Some(false)).count();
  let w = canvas.iter().filter(|&&v|v==Some(true)).count();
  (ans, if ans {Some((b,w))} else {None})
}
