use std::io::{self, BufRead};

#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct Mountain {
  name: String,
  height: usize,
}

fn main() {
  let mut lines = io::stdin().lock().lines();
  let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

  let mut mountains: Vec<Mountain> = vec![];
  for _ in 0..n {
    let name_and_height: Vec<String> = lines
      .next()
      .unwrap()
      .unwrap()
      .split_whitespace()
      .map(|s| String::from(s))
      .collect();

    mountains.push(Mountain {
      name: name_and_height[0].to_string(),
      height: name_and_height[1].parse().unwrap(),
    });
  }

  // Sort moutains by height
  mountains.sort_by(|a, b| b.height.cmp(&a.height));

  println!("{}", mountains[1].name);
}
