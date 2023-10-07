use proconio::*;

fn main() {
  input! {
    s:String
  }

  let s_char: Vec<char> = s.chars().collect();

  for i in 1..16 {
    if i % 2 == 1 && s_char[i] != '0' {
      println!("No");
      return;
    }
  }

  println!("Yes");
}
