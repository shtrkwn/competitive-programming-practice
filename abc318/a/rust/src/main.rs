use proconio::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    p:usize
  }
  // n = ans * p + あまり
  if n >= m {
    let ans = (n - m) / p + 1;
    println!("{}", ans);
  } else {
    println!("0");
  }
}
