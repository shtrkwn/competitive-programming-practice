use proconio::*;

fn main() {
  input! {
    m: usize,
    d: [usize;m]
  }
  let middle_day = (d.iter().fold(0, |acc, x| acc + x) + 1) / 2;
  let mut count = 1;
  for month in 0..m {
    for day in 1..(d[month] + 1) {
      if count == middle_day {
        println!("{} {}", month + 1, day);
        return;
      } else {
        count += 1;
      }
    }
  }
}
