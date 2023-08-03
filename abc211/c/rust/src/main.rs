use proconio::*;

fn main() {
  input! {
    s: String
  };

  let mut dp: Vec<usize> = vec![0; 8];
  let pattern = ['c', 'h', 'o', 'k', 'u', 'd', 'a', 'i'];
  let div: usize = 1000000007;

  for c in s.chars() {
    match pattern.iter().position(|p| p == &c) {
      Some(i) => {
        if i == 0 {
          dp[0] += 1;
        } else {
          dp[i] = dp[i] + dp[i - 1];
        }
        dp[i] %= div;
      }
      None => {}
    };
  }

  println!("{}", dp[7]);
}
