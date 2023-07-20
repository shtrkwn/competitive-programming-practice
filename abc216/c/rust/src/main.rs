use proconio::input;

fn main() {
  input! {
    n:u64
  }
  let ans: Vec<&str> = format!("{:b}", n)
    .chars()
    .map(|s| if s == '1' { "BA" } else { "B" })
    .collect();

  let ans = ans.join("");

  println!("{}", ans);
}
