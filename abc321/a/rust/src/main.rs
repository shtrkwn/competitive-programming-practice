use proconio::*;

fn main() {
  input! {
    n: String
  }

  let n_chars = n.chars();

  let mut last_num = 10;

  for c in n_chars {
    let c: usize = c.to_string().parse().unwrap();
    if c >= last_num {
      println!("No");
      return;
    }
    last_num = c;
  }
  println!("Yes");
}
