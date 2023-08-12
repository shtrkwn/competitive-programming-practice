use proconio::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    s:String,
    c: [usize;n]
  }

  // 入力補正
  let c: Vec<usize> = c.iter().map(|x| x - 1).collect();

  // colors[m][色id]
  let mut colors: Vec<Vec<char>> = Vec::new();
  for _ in 0..m {
    colors.push(Vec::new());
  }

  // 文字列を文字配列に分解
  let s_chars: Vec<char> = s.chars().collect();

  for i in 0..n {
    let color_i = c[i];
    colors[color_i].push(s_chars[i]);
  }

  let mut cursors = vec![0; m];
  for i in 0..m {
    cursors[i] = colors[i].len() - 1;
  }

  let mut ans = String::new();
  for i in 0..n {
    let color_i = c[i];
    let cursor = cursors[color_i] % colors[color_i].len();

    ans.push_str(&format!("{}", colors[color_i][cursor]));

    cursors[color_i] += 1;
  }

  println!("{}", ans);
}
