use proconio::*;

fn main() {
  input! {
    k:usize
  }

  let mut ans = vec!['A'; k];

  for i in 0..k {
    ans[i] = std::char::from_u32(65 + i as u32).unwrap();
  }

  let ans: String = ans.into_iter().collect();
  println!("{}", ans);
}
