use proconio::input;

fn main() {
  input! {
    n: usize,
    mut c: [usize;n]
  }

  let mut ans = 1;
  let div = 1000000000 + 7;
  c.sort();

  for i in 0..n {
    ans *= c[i] - i;
    ans %= div;
  }

  println!("{}", ans);
}
