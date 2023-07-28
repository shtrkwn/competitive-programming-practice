use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
  input! {
      n: usize,
      k: usize,
      p: [usize; n],
  }

  let mut que: BinaryHeap<Reverse<usize>> = p[0..k].iter().map(|&x| Reverse(x)).collect();
  println!("{}", que.peek().unwrap().0);

  for i in k..n {
    let minima = que.pop().unwrap().0.max(p[i]);
    que.push(Reverse(minima));

    println!("{}", que.peek().unwrap().0);
  }
}
