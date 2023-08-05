use proconio::*;

fn main() {
  input! {
    n:usize,
    p: [usize;n]
  }

  let mut p_c = p.clone();
  p_c.sort();

  if p[0] == p_c[n - 1] {
    if n > 2 && p_c[n - 2] == p_c[n - 1] {
      println!("{}", 1);
    } else {
      println!("{}", 0);
    }
  } else {
    println!("{}", p_c[n - 1] - p[0] + 1);
  }
}
