use proconio::*;

fn main() {
  input! {
    n:usize
  }

  let mut graph: Vec<Vec<usize>> = Vec::new();
  for _ in 0..n {
    input! {
      c: usize,
      p: [usize; c]
    }
    graph.push(p.into_iter().map(|x| x - 1).collect());
  }

  match topological_sort(&graph, 0) {
    Some(order) => {
      for el in order.iter().rev() {
        if el == &0 {
          break;
        }
        print!("{} ", el + 1);
      }
      println!();
    }
    None => println!("The graph is not a DAG!"),
  }
}
fn dfs(graph: &Vec<Vec<usize>>, start: usize, visited: &mut Vec<bool>) -> Vec<usize> {
  let mut stack = vec![start];
  let mut component = vec![];

  while let Some(node) = stack.pop() {
    if !visited[node] {
      visited[node] = true;
      component.push(node);
      for &next in &graph[node] {
        if !visited[next] {
          stack.push(next);
        }
      }
    }
  }

  component
}

fn topological_sort(graph: &Vec<Vec<usize>>, start: usize) -> Option<Vec<usize>> {
  let mut visited = vec![false; graph.len()];
  let component = dfs(graph, start, &mut visited);

  let mut indegree = vec![0; graph.len()];

  for &node in &component {
    for &next in &graph[node] {
      indegree[next] += 1;
    }
  }

  let mut queue: Vec<usize> = component
    .clone()
    .into_iter()
    .filter(|&i| indegree[i] == 0)
    .collect();
  let mut sorted = Vec::new();

  while let Some(node) = queue.pop() {
    sorted.push(node);
    for &next in &graph[node] {
      indegree[next] -= 1;
      if indegree[next] == 0 {
        queue.push(next);
      }
    }
  }

  if sorted.len() == component.len() {
    Some(sorted)
  } else {
    None
  }
}
