use std::io;

fn main() {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();

  let (a, b): (f64, f64) = {
    let mut split = input.trim().split_whitespace();
    let a: f64 = split.next().unwrap().parse().unwrap();
    let b: f64 = split.next().unwrap().parse().unwrap();
    (a, b)
  };

  let ans = b / 100.0 * a;
  println!("{}", ans);
}
