use proconio::*;

fn main() {
  input! {
    mut n:usize,
    d:[usize;n*(n-1)/2]
  }

  let mut edges: Vec<Vec<usize>> = vec![vec![]; n - 1];
  let mut counter = n - 1;
  let mut node_index = 0;
  for i in 0..d.len() {
    edges[node_index].push(d[i]);
    counter -= 1;
    if counter == 0 {
      counter = (n - 1) - node_index - 1;
      node_index += 1;
    }
  }

  // nが奇数の場合使わないノードを可変にするために、重み0のノードを追加
  if n % 2 == 1 {
    n += 1;
    for i in 0..edges.len() {
      edges[i].push(0);
    }
    edges.push(vec![0]);
  }
  let ans = dfs(&edges, &mut vec![false; n], 0);
  println!("{}", ans);
}

fn dfs(edges: &Vec<Vec<usize>>, used_node: &mut Vec<bool>, start: usize) -> usize {
  let mut max = 0;
  for node1_id in start..edges.len() {
    if used_node[node1_id] {
      continue;
    }
    used_node[node1_id] = true;

    for j in 0..edges[node1_id].len() {
      let node2_id = node1_id + 1 + j;
      if used_node[node2_id] {
        continue;
      }

      used_node[node2_id] = true;
      max = (edges[node1_id][j] + dfs(edges, used_node, node1_id + 1)).max(max);
      used_node[node2_id] = false;
    }
    used_node[node1_id] = false;
  }

  return max;
}
