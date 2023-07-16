use std::io;

fn main() {
  let stdin = io::stdin();
  let mut input = String::new();

  stdin.read_line(&mut input).unwrap();
  let x: usize = input.trim().parse().unwrap();

  if x < 40 {
    println!("{}", 40 - x);
  } else if x < 70 {
    println!("{}", 70 - x);
  } else if x < 90 {
    println!("{}", 90 - x);
  } else {
    println!("{}", "expert")
  }
}
