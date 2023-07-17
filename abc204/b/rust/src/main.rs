use std::io::{self, BufRead};

fn main() {
  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();

  let _n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

  let a: Vec<i32> = lines
    .next()                       // 次の行を取得
    .unwrap()                     // Option型からResult型を取得
    .unwrap()                     // Result型からString型を取得
    .split_whitespace()           // 空白文字によって、Split
    .map(|s| s.parse().unwrap())  // 文字を整数にパース
    .collect();                   // Vec<i32>のコレクションに変換

  let mut num_nuts = 0;
  for n in a {
    if n > 9 {
      num_nuts += n - 10;
    }
  }
  println!("{}", num_nuts);
}
