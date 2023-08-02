use proconio::*;

fn main() {
  input! {
    s: String
  };

  let s_chars: Vec<char> = s.chars().collect();
  let n = s_chars.len();
  let mut dp: Vec<Vec<usize>> = vec![vec![0; s.len()]; 8];
  let pattern = ['c', 'h', 'o', 'k', 'u', 'd', 'a', 'i'];
  let div: usize = 1000000007;

  // 初期化
  for j in 0..n {
    if s_chars[j] == 'c' {
      dp[0][j] = 1;
    }
  }

  for i in 1..8 {
    for j in 0..n {
      if s_chars[j] == pattern[i] {
        dp[i][j] = dp[i - 1][0..j].iter().fold(0, |acc, x| acc + x) % div;
      }
    }
  }

  println!("{}", dp[7].iter().fold(0, |acc, &x| acc + x) % div);
}
