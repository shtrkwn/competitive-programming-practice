use proconio::input;

fn main() {
  input! {
    n: usize,
    d: usize,
    s: [String; n]
  }

  let s: Vec<Vec<char>> = s.iter().map(|s| s.chars().collect()).collect();

  let mut num_com_days_off = vec![0; d];

  for i in 0..n {
    for j in 0..d {
      if s[i][j] == 'o' {
        num_com_days_off[j] += 1;
      }
    }
  }

  let mut counter = 0;
  let mut max = 0;
  for j in 0..d {
    if num_com_days_off[j] == n {
      counter += 1;
    } else {
      counter = 0;
    }

    if max < counter {
      max = counter;
    }
  }
  println!("{}", max);
}
