use std::io;

fn f(t: i32) -> i32 {
  return t * t + 2 * t + 3;
}

fn main() {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  let t: i32 = input.trim().parse().unwrap();

  println!("{}", f(f(f(t) + t) + f(f(t))));
}
