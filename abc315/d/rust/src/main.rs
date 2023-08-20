use proconio::*;
use std::collections::{HashMap, HashSet};

fn init_counter(
  c: &Vec<Vec<char>>,
  main_dim: usize,
  sub_dim: usize,
  is_horizontal: bool,
) -> Vec<HashMap<char, usize>> {
  let mut counter: Vec<HashMap<char, usize>> = vec![HashMap::new(); main_dim];
  for main in 0..main_dim {
    for sub in 0..sub_dim {
      let cij = if is_horizontal {
        c[main][sub]
      } else {
        c[sub][main]
      };
      *counter[main].entry(cij).or_insert(0) += 1;
    }
  }
  counter
}

fn mark_for_deletion(
  counter: &mut Vec<HashMap<char, usize>>,
  c: &Vec<Vec<char>>,
  main_dim: usize,
  sub_dim: usize,
  is_horizontal: bool,
  marked: &mut HashSet<(usize, usize)>,
) {
  for main in 0..main_dim {
    if counter[main].len() == 1 {
      let (&deletee, &size) = counter[main].iter().next().unwrap();
      if size > 1 {
        counter[main].remove(&deletee);
        for sub in 0..sub_dim {
          if (is_horizontal && c[main][sub] == deletee)
            || (!is_horizontal && c[sub][main] == deletee)
          {
            marked.insert(if is_horizontal {
              (main, sub)
            } else {
              (sub, main)
            });
          }
        }
      }
    }
  }
}

fn main() {
  input! {
      h: usize,
      w: usize,
      s: [String;h]
  }

  let mut c: Vec<Vec<char>> = s.into_iter().map(|str| str.chars().collect()).collect();

  let mut counter_horizontal = init_counter(&c, h, w, true);
  let mut counter_vertical = init_counter(&c, w, h, false);

  loop {
    let mut marked: HashSet<(usize, usize)> = HashSet::new();

    mark_for_deletion(&mut counter_horizontal, &c, h, w, true, &mut marked);
    mark_for_deletion(&mut counter_vertical, &c, w, h, false, &mut marked);

    if !marked.is_empty() {
      for (i, j) in &marked {
        let deletee = c[*i][*j];
        c[*i][*j] = '.';

        if let Some(val) = counter_horizontal[*i].get_mut(&deletee) {
          *val = val.saturating_sub(1);
          if *val == 0 {
            counter_horizontal[*i].remove(&deletee);
          }
        }

        if let Some(val) = counter_vertical[*j].get_mut(&deletee) {
          *val = val.saturating_sub(1);
          if *val == 0 {
            counter_vertical[*j].remove(&deletee);
          }
        }
      }
    } else {
      break;
    }
  }

  let mut ans = 0;
  for i in 0..h {
    for j in 0..w {
      if c[i][j] != '.' {
        ans += 1;
      }
    }
  }

  println!("{}", ans);
}
