use std::io::{self, BufRead};

fn main() {
  let mut lines = io::stdin().lock().lines();

  let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
  let mut xy: Vec<Vec<isize>> = vec![];

  for _ in 0..n {
    xy.push(
      lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect(),
    )
  }

  println!("{:?}", xy);
  let mut max_distance_powered = 0;
  for i in 0..n {
    for j in (i + 1)..n {
      let x = xy[i][0] - xy[j][0];
      let y = xy[i][1] - xy[j][1];
      let distance_powered = x * x + y * y;
      if max_distance_powered < distance_powered {
        max_distance_powered = distance_powered;
      }
    }
  }

  println!("{}", (max_distance_powered as f64).sqrt());
}
