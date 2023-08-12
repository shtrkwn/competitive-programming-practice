use proconio::*;

fn main() {
  input! {
      n: usize
  }

  let s = "1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679".to_string();

  let chars: Vec<char> = s.chars().collect();
  print!("3.");
  for i in 0..n {
    print!("{}", chars[i]);
  }
  println!();
}
