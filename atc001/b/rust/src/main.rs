use proconio::*;

struct UnionFind {
  parent: Vec<usize>,
  rank: Vec<usize>,
}

impl UnionFind {
  pub fn new(n: usize) -> UnionFind {
    UnionFind {
      parent: (0..n).collect(),
      rank: vec![0; n],
    }
  }

  pub fn find(&mut self, x: usize) -> usize {
    if self.parent[x] == x {
      x
    } else {
      let parent = self.parent[x];
      self.parent[x] = self.find(parent);
      self.parent[x]
    }
  }

  pub fn union(&mut self, x: usize, y: usize) {
    let x_root = self.find(x);
    let y_root = self.find(y);
    if x_root != y_root {
      if self.rank[x_root] < self.rank[y_root] {
        self.parent[x_root] = y_root;
      } else if self.rank[x_root] > self.rank[y_root] {
        self.parent[y_root] = x_root;
      } else {
        self.parent[y_root] = x_root;
        self.rank[x_root] += 1;
      }
    }
  }
}

fn main() {
  input! {
    n: usize,
    q: usize,
    queries: [[usize;3];q]
  }

  let mut union_find = UnionFind::new(n);

  for query in queries {
    if query[0] == 0 {
      union_find.union(query[1], query[2]);
    } else {
      let x_root = union_find.find(query[1]);
      let y_root = union_find.find(query[2]);
      if x_root == y_root {
        println!("Yes")
      } else {
        println!("No")
      }
    }
  }
}
