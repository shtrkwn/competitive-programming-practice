use proconio::*;

fn main() {
  input! {
    n: usize
  }

  let mut ans: Vec<(usize, usize)> = vec![];

  for i in 0..n {
    input! {
      s:String
    }
    let mut count = 0;
    for c in s.chars() {
      if c == 'o' {
        count += 1;
      }
    }
    ans.push((i, count));
  }

  ans.sort_by(|x1, x2| {
    if x1.1 == x2.1 {
      return x1.0.cmp(&x2.0);
    }
    x2.1.cmp(&x1.1)
  });

  for a in ans {
    print!("{} ", (a.0 + 1));
  }
}
