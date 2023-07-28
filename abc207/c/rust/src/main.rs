use proconio::input;

fn main() {
  input! {
    n: usize,
    tlr: [[usize;3];n]
  }

  let ranges: Vec<(f64, f64)> = tlr
    .into_iter()
    .map(|x| match x[0] % 4 {
      1 => (x[1] as f64, x[2] as f64),
      2 => (x[1] as f64, x[2] as f64 - 0.1),
      3 => (x[1] as f64 + 0.1, x[2] as f64),
      0 => (x[1] as f64 + 0.1, x[2] as f64 - 0.1),
      _ => unreachable!(),
    })
    .collect();

  let mut ans = 0;
  for i in 0..ranges.len() {
    for j in (i + 1)..ranges.len() {
      if ranges[i].0.max(ranges[j].0) <= ranges[i].1.min(ranges[j].1) {
        ans += 1;
      }
    }
  }

  println!("{}", ans);
}
