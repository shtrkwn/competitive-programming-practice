use proconio::input;

fn main() {
  input! {
    n: usize,
    ai: [u64; n],
    x: u64
  }

  let sum_ai = ai.iter().fold(0, |acc, ak| acc + ak);
  let repeat = x / sum_ai;
  let comp = x - sum_ai * repeat;

  let mut sum_ai = 0;
  for i in 0..n {
    if comp < sum_ai + ai[i] {
      println!("{}", n as u64 * repeat + (i + 1) as u64);
      return;
    }
    sum_ai += ai[i];
  }
}
