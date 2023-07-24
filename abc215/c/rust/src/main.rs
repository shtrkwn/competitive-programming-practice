use proconio::input;
use std::collections::HashSet;

fn main() {
  input! {
   s: String,
   k: usize
  }

  let chars: Vec<char> = s.trim().chars().collect();
  let is_char_usable_state: Vec<bool> = chars.iter().map(|_| true).collect();
  let mut set: HashSet<String> = HashSet::new();

  find_string_by_dfs("".to_string(), &chars, &is_char_usable_state, &mut set);
  let mut strings: Vec<String> = set.into_iter().collect();
  strings.sort();
  println!("{}", strings[k - 1]);
}

fn find_string_by_dfs(
  prefix: String,
  chars: &Vec<char>,
  is_char_usable_state: &Vec<bool>,
  set: &mut HashSet<String>,
) {
  if is_char_usable_state.iter().all(|&val| !val) {
    set.insert(prefix);
  } else {
    for i in 0..is_char_usable_state.len() {
      if is_char_usable_state[i] {
        let mut is_char_usable_state = is_char_usable_state.clone();
        is_char_usable_state[i] = false;
        let mut prefix = String::from(&prefix);
        prefix.push(chars[i]);
        find_string_by_dfs(prefix, chars, &is_char_usable_state, set);
      }
    }
  }
}
