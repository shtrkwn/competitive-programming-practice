use proconio::*;

fn main() {
  input! {
      mut s: String
  }
  let ans = s
    .replace("a", "")
    .replace("e", "")
    .replace("i", "")
    .replace("o", "")
    .replace("u", "");
  println!("{}", ans);
}
