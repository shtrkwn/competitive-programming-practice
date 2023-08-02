use proconio::*;

fn main() {
  input! {
    n:usize,
    s: String
  }

  let s: Vec<char> = s.chars().collect();

  // R,Lどちらが指定されても、iが左右どちらになるかは未確定（次の指示に依存する）
  // Rが指定された場合、iより左側は確定（i-1も右側で確定）
  // Lが指定された場合、iより右側は確定（i-1も左側で確定）

  let mut left: Vec<usize> = Vec::with_capacity(n);
  let mut middle = 0;
  let mut right: Vec<usize> = Vec::with_capacity(n);

  for i in 0..n {
    if s[i] == 'L' {
      right.push(middle);
      middle = i + 1;
    } else {
      left.push(middle);
      middle = i + 1;
    }
  }

  left.push(middle);
  left.extend(right.into_iter().rev());

  let left: String = left.into_iter().map(|u| format!("{} ", u)).collect();
  println!("{}", left.trim());
}
