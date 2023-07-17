use std::io::{self, BufRead};

fn main() {
  let mut lines = io::stdin().lock().lines();

  let mut s: Vec<String> = Vec::new();
  for _ in 0..3 {
    s.push(lines.next().unwrap().unwrap().trim().to_string());
  }
  let t = lines.next().unwrap().unwrap().trim().to_string();

  let mut ans = String::new();
  for c in t.chars() {
    ans.push_str(&s[(c.to_digit(10).unwrap() - 1) as usize]);
  }
  println!("{}", &ans)
}
