use proconio::input;
use std::vec::Vec;

fn dfs(v: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
  if visited[v] {
    return;
  }
  visited[v] = true;
  for &vv in &graph[v] {
    dfs(vv, graph, visited);
  }
}

fn main() {
  input! {
  n: usize,
  m: usize,
  roads: [(usize, usize); m]
  }

  let mut graph = vec![vec![]; n];
  for (a, b) in roads {
    graph[a - 1].push(b - 1);
  }

  let mut ans = 0;
  for i in 0..n {
    let mut visited = vec![false; n];
    dfs(i, &graph, &mut visited);
    ans += visited.iter().filter(|&&x| x).count();
  }

  println!("{}", ans);
}
