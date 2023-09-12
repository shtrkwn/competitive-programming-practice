use proconio::*;

fn main() {
  input! {
    n: usize
  }

  let mut ans = String::new();

  for i in 0..(n + 1) {
    let mut ans_tmp = String::from("-");
    for j in 1..10 {
      if n % j != 0 {
        continue;
      }
      if i % (n / j) == 0 {
        ans_tmp = format!("{}", j).to_string();
        break;
      }
    }
    ans.push_str(&ans_tmp);
  }

  println!("{}", ans);
}
