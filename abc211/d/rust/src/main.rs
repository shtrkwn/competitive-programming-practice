use proconio::*;
use std::collections::VecDeque;

struct Graph {
  adj_list: Vec<Vec<usize>>,
}

impl Graph {
  // 新しいグラフを初期化
  fn new(n: usize) -> Self {
    Graph {
      adj_list: vec![vec![]; n],
    }
  }

  // グラフにエッジを追加
  fn add_edge(&mut self, u: usize, v: usize) {
    self.adj_list[u].push(v);
  }

  // 幅優先探索を使用して、スタートからの最短距離と各ノードへのルート数を計算
  fn bfs(&self, start: usize) {
    let n = self.adj_list.len();
    let mut visited = vec![false; n];
    let mut distance_from_start = vec![0; n];
    let mut num_routes = vec![0; n];
    let mut queue = VecDeque::new();
    visited[start] = true;
    distance_from_start[start] = 0;
    num_routes[start] = 1;
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
      for &neighbour in &self.adj_list[current] {
        if !visited[neighbour] || distance_from_start[neighbour] == distance_from_start[current] + 1
        {
          if !visited[neighbour] {
            queue.push_back(neighbour);
          }
          visited[neighbour] = true;
          distance_from_start[neighbour] = distance_from_start[current] + 1;
          num_routes[neighbour] = (num_routes[neighbour] + num_routes[current]) % 1_000_000_007;
        }
      }
    }

    let destination = n - 1;
    println!("{}", num_routes[destination]);
  }
}

fn main() {
  input! {
      n:usize,
      m:usize,
      roads:[(usize, usize); m]
  }

  let mut graph = Graph::new(n);

  for (city1, city2) in roads {
    graph.add_edge(city1 - 1, city2 - 1);
    graph.add_edge(city2 - 1, city1 - 1);
  }

  graph.bfs(0);
}
