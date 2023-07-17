use std::{
  collections::HashSet,
  io::{self, BufRead},
};

fn main() {
  let mut lines = io::stdin().lock().lines();
  let _: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
  let unique_ints: HashSet<usize> = lines
    .next()
    .unwrap()
    .unwrap()
    .split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();

  println!("{}", unique_ints.len());
}
