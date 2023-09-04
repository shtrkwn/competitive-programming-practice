use proconio::*;

#[derive(Clone, Debug)]
struct EdgeGroup {
  dest: usize,
  weight: usize,
}

fn main() {
  input! {
    n:usize,
    d:[usize;n*(n-1)/2]
  }

  let mut edges: Vec<Vec<usize>> = vec![vec![]; n - 1];
  let mut counter = n;
  let mut node_index = 0;
  for i in 0..d.len() {
    edges[node_index].push(d[i]);
    counter -= 1;
    if counter == 0 {
      counter = n - node_index - 1;
      node_index += 1;
    }
  }

  // EdgeGroupに入れ替え
  let mut edge_groups: Vec<Vec<EdgeGroup>> = vec![vec![]; n - 1];
  for i in 0..(n - 1) {
    for j in 0..edges[i].len() {
      edge_groups[i].push(EdgeGroup {
        dest: j + i,
        weight: edges[i][j],
      })
    }
    edge_groups[i].sort_by(|x1, x2| x2.weight.cmp(&x1.weight));
  }

  let ans = dfs(&edge_groups, &mut vec![false; n]);
  println!("{}", ans);
}

fn dfs(edge_groups: &Vec<Vec<EdgeGroup>>, used_node: &mut Vec<bool>) -> usize {
  let mut max = 0;
  for i in 0..edge_groups.len() {
    if used_node[i] {
      continue;
    }
    used_node[i] = true;

    for j in 0..edge_groups[i].len() {
      if !used_node[edge_groups[i][j].dest] {
        used_node[edge_groups[i][j].dest] = true;
        // println!("{:?}", used_node);
        max = (edge_groups[i][j].weight + dfs(edge_groups, used_node)).max(max);
        used_node[edge_groups[i][j].dest] = false;
        continue;
      }
    }
    used_node[i] = false;
  }

  return max;
}
