use proconio::*;

fn main() {
  input! {
    s: String,
    k: usize
  }

  let s_chars: Vec<char> = s.chars().collect();

  // 最初のk+1番目の"."を見つける
  let mut current_terminal_index = s_chars
    .iter()
    .enumerate()
    .filter(|&(_, &c)| c == '.')
    .nth(k)
    .map_or(s_chars.len(), |(i, _)| i - 1);

  if current_terminal_index == s_chars.len() {
    println!("{}", s_chars.len());
    return;
  }

  let mut max_len = current_terminal_index + 1;
  for i in 1..s_chars.len() {
    // 左がXである場合、左の文字から繋げたほうが長いので、この文字列が最大になることはない
    if s_chars[i - 1] == 'X' {
      continue;
    }
    for j in (current_terminal_index + 2)..s_chars.len() {
      if s_chars[j] == '.' {
        current_terminal_index = j - 1;
        break;
      }
      if j == s_chars.len() - 1 {
        current_terminal_index = j;
      }
    }
    max_len = max_len.max(current_terminal_index + 1 - i);
    if current_terminal_index == s_chars.len() - 1 {
      break;
    }
  }
  println!("{}", max_len);
}
