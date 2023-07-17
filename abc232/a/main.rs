use std::io::{self, BufRead};

fn main() {
  let mut lines = io::stdin().lock().lines();

  let s: String = String::from(lines.next().unwrap().unwrap().trim());
  let ab: Vec<usize> = s.split('x').map(|s| s.parse().unwrap()).collect();

  println!("{}", ab[0]*ab[1]);
}
