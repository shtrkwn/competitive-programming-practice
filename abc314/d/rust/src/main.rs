use proconio::*;
fn main() {
  input! {
    _: usize,
    s: String,
    q: usize,
    queries: [(usize, usize, char);q]
  }

  // ■方針
  // 大文字小文字の変更は一番最後のもののみ適用するため、それ以外の変更は無視する
  // 文字の入れ替えはすべて実施する

  // 最後に実施する大文字小文字のクエリの位置を特定する
  let mut last_case_change_position: Option<usize> = Option::None;
  for (i, query) in queries.iter().enumerate() {
    if query.0 == 2 || query.0 == 3 {
      last_case_change_position = Some(i);
    }
  }

  // 回答文字列
  let mut ans: Vec<char> = s.chars().collect();

  for (i, query) in queries.iter().enumerate() {
    if query.0 == 2 || query.0 == 3 {
      if i == last_case_change_position.unwrap() {
        let mut s: String = ans.iter().collect();
        if query.0 == 2 {
          s = s.to_lowercase();
        } else {
          s = s.to_uppercase();
        }
        ans = s.chars().collect();
      }
    } else {
      let x = query.1 - 1;
      let c = query.2;
      ans[x] = c;
    }
  }

  let ans: String = ans.iter().collect();
  println!("{}", ans);
}
