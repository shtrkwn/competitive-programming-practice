use proconio::input;

fn main() {
  input! {
    n: usize,
    sin: String
  }

  let mut judge = [false, false, false];
  let mut counter: usize = 0;
  let s: Vec<char> = sin.chars().collect();

  for i in 0..n {
    if s[i] == 'A' {
      judge[0] = true;
    } else if s[i] == 'B' {
      judge[1] = true;
    } else if s[i] == 'C' {
      judge[2] = true;
    }

    if judge[0] && judge[1] && judge[2] {
      counter = i;
      break;
    }
  }
  println!("{}", counter + 1);
}
