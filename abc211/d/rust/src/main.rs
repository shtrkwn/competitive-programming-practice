use proconio::*;
use std::collections::VecDeque;

struct Graph {
  adj_list: Vec<Vec<usize>>,
}

impl Graph {
  fn new(n: usize) -> Self {
    Graph {
      adj_list: vec![vec![]; n],
    }
  }

  fn add_edge(&mut self, u: usize, v: usize) {
    self.adj_list[u].push(v);
  }

  fn bfs(&self, start: usize) {
    let n = self.adj_list.len();
    let mut visited = vec![false; n];
    let mut shortes_arrivals = vec![0; n];
    let mut num_routes = vec![0; n];
    let mut queue = VecDeque::new();
    visited[start] = true;
    shortes_arrivals[start] = 0;
    num_routes[start] = 1;
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
      for &neighbour in &self.adj_list[current] {
        if !visited[neighbour] {
          visited[neighbour] = true;
          shortes_arrivals[neighbour] = shortes_arrivals[current] + 1;
          num_routes[neighbour] += num_routes[current];
          queue.push_back(neighbour);
        } else if shortes_arrivals[neighbour] == shortes_arrivals[current] + 1 {
          num_routes[neighbour] += num_routes[current];
        }
      }
    }
    // println!("{:?}", num_routes);
    println!("{}", num_routes[n - 1]);
  }
}

fn main() {
  input! {
    n:usize,
    m:usize,
    roads:[(usize, usize);m]
  }

  let mut graph = Graph::new(n);

  // citiesの初期化
  for (city1, city2) in roads {
    graph.add_edge(city1 - 1, city2 - 1);
    graph.add_edge(city2 - 1, city1 - 1);
  }

  graph.bfs(0);
}
