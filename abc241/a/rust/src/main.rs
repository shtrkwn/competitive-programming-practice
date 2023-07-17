use std::io::{self, BufRead};

fn main() {
  let mut lines = io::stdin().lock().lines();

  let ak: Vec<usize> = lines
    .next()
    .unwrap()
    .unwrap()
    .trim()
    .split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();

  println!("{}", ak[ak[ak[0]]]);
}
