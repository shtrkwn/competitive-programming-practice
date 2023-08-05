use proconio::*;

fn main() {
  input! {
    n: usize,
    mut a: [usize;n]
  }

  a.sort();
  let sum = a.iter().fold(0, |acc, x| acc + x);
  let max = sum / n + 1;
  let min = sum / n;
  let num_maxs = sum - min * n;
  let mut target: Vec<usize> = Vec::new();
  target.extend(vec![min; n - num_maxs]);
  target.extend(vec![max; num_maxs]);

  let mut ans = 0;
  for i in 0..n {
    if a[i] < target[i] {
      ans += target[i] - a[i];
    } else {
      println!("{}", ans);
      return;
    }
  }
}
