use std::io::{self, BufRead};

fn main() {
  let mut lines = io::stdin().lock().lines();

  let mut points: Vec<Vec<isize>> = vec![];

  for _ in 0..3 {
    points.push(
      lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|d| d.parse().unwrap())
        .collect(),
    );
  }

  let mut xans: isize = 0;
  let mut yans: isize = 0;

  if points[0][0] == points[1][0] {
    xans = points[2][0];
  } else if points[0][0] == points[2][0] {
    xans = points[1][0];
  } else if points[1][0] == points[2][0] {
    xans = points[0][0];
  }

  if points[0][1] == points[1][1] {
    yans = points[2][1];
  } else if points[0][1] == points[2][1] {
    yans = points[1][1];
  } else if points[1][1] == points[2][1] {
    yans = points[0][1];
  }

  println!("{} {}", xans, yans);
}
