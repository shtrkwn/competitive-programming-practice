use std::collections::HashMap;

use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize;n]
  };

  let mut map: HashMap<usize, usize> = HashMap::new();

  let mut ans: usize = 0;
  for i in (1..n).rev() {
    *map.entry(a[i]).or_insert(0) += 1;

    ans += n - i - *map.get(&a[i - 1]).unwrap_or(&0);
  }

  println!("{}", ans);
}
