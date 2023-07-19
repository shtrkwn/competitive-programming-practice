use proconio::input;

fn main() {
  input! {
      a: i32,
      b: i32,
      c: i32
  }

  let ans = if a >= 0 && b >= 0 {
    if a < b {
      "<"
    } else if a == b {
      "="
    } else {
      ">"
    }
  } else {
    if c % 2 == 0 {
      if a.abs() < b.abs() {
        "<"
      } else if a == -b {
        "="
      } else {
        ">"
      }
    } else if a < b {
      "<"
    } else {
      // a^c > 0 && b^c < 0
      ">"
    }
  };
  println!("{}", ans);
}
