use std::collections::HashSet;

use proconio::*;

fn main() {
  input! {
    t:usize
  }

  for i in 0..t {
    input! {
      n:usize,
      x:usize,
      k:usize
    }

    if k > log2(&n) + 1 {
      println!("0");
      return;
    }

    let mut ans: usize = 0;

    let mut current_layer = log2(&k);
    let visited: HashSet<usize> = HashSet::new();
    for i in 0..k {
      let left = (x / (2 ^ i)) * 2 ^ (k - i);
      let right = 0;
      if left < right {
        ans += right - left;
      }
    }
  }

  println!("Hello, world!");
}

fn log2(x: &usize) -> usize {
  let mut val = *x;
  let mut count = 0;

  while val > 1 {
    val /= 2;
    count += 1;
  }
  count
}
