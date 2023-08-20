use petgraph::{
  algo::{toposort, DfsSpace},
  graph::{DiGraph, NodeIndex},
  visit::Dfs,
};
use proconio::*;
use std::collections::HashSet;

fn main() {
  input! {
      n: usize,
  }
  // initi graph
  let mut g: DiGraph<(), ()> = DiGraph::new();

  // add nodes to the graph
  (0..n).into_iter().for_each(|_| {
    g.add_node(());
  });

  // add edges to the graph
  for start in 0..n {
    input! {
      c: usize,
      edges: [usize;c]
    }
    for end in edges {
      let start = NodeIndex::from(start as u32);
      let end = NodeIndex::from(end as u32 - 1);
      g.add_edge(start, end, ());
    }
  }

  let mut space = DfsSpace::new(&g);
  match toposort(&g, Some(&mut space)) {
    Ok(result) => {
      let reachable = reachable_nodes(&g, 0.into());
      let order: Vec<usize> = result
        .into_iter()
        .filter(|i| reachable.contains(&i) && !i.eq(&NodeIndex::from(0 as u32)))
        .map(|index| index.index() + 1)
        .rev()
        .collect();
      for node in order {
        print!("{} ", node);
      }
      println!();
    }
    Err(_) => (),
  }
}

fn reachable_nodes(graph: &DiGraph<(), ()>, start: NodeIndex) -> HashSet<NodeIndex> {
  let mut dfs = Dfs::new(graph, start);
  let mut reachable = HashSet::new();

  while let Some(node) = dfs.next(graph) {
    reachable.insert(node);
  }
  reachable
}
