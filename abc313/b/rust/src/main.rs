use std::collections::HashSet;

use proconio::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    ab: [(usize, usize);m]
  }

  let mut p: Vec<Vec<usize>> = vec![Vec::new(); n];
  for (a, b) in ab {
    p[a - 1].push(b - 1);
  }

  // すべてのpに対してDFS
  for i in 0..n {
    let mut set: HashSet<usize> = HashSet::new();
    dfs(&p, i, &mut set);
    let rank = set.len();
    if rank == n - 1 {
      println!("{}", i + 1);
      return;
    }
  }
  println!("-1");
}

fn dfs(p: &Vec<Vec<usize>>, id: usize, set: &mut HashSet<usize>) {
  for &child_id in p[id].iter() {
    if !set.contains(&child_id) {
      set.insert(child_id);
      dfs(p, child_id, set)
    }
  }
}
