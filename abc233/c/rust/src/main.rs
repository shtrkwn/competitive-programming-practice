use proconio::*;

fn main() {
  input! {
    n: usize,
    x: u64,
  }
  let mut a: Vec<Vec<usize>> = Vec::new();

  for _ in 0..n {
    input! {
      l: usize,
      _a: [usize;l]
    }
    a.push(_a);
  }

  let num_combo = calc_num_combo(&a, 0, x);
  println!("{}", num_combo);
}

fn calc_num_combo(bag: &Vec<Vec<usize>>, bag_pointer: usize, product: u64) -> u64 {
  let mut ans = 0 as u64;
  for b in &bag[bag_pointer] {
    if product % (*b as u64) == 0 {
      if bag_pointer < bag.len() - 1 {
        ans += calc_num_combo(bag, bag_pointer + 1, product / (*b as u64));
      } else {
        if product == *b as u64 {
          ans += 1;
        }
      }
    }
  }
  ans
}
