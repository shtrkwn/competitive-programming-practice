use proconio::input;

fn main() {
  input! {
      n: usize,
      q: usize,
      mut a: [usize;n],
      x: [usize;q]
  }

  a.sort();

  for &xq in &x {
    match a.binary_search(&xq) {
      Ok(index) => println!("{}", a.len() - index),
      Err(index) => println!("{}", a.len() - index),
    }
  }
}
