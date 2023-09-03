use proconio::*;

fn main() {
  input! {
      n:usize,
      d:usize,
      fee_with_ticket:usize,
      mut fees: [usize;n]
  }

  fees.sort_by(|x1, x2| x2.cmp(x1));

  let num_groups = (n - 1) / d + 1;
  let mut fees_for_calc: Vec<Vec<usize>> = vec![vec![0; d]; num_groups];
  for i in 0..n {
    fees_for_calc[i / d][i % d] = fees[i];
  }
  let fees_for_comp: Vec<usize> = fees_for_calc
    .into_iter()
    .map(|fees| fees.iter().sum())
    .collect();

  let mut ans = 0;
  for i in 0..num_groups {
    if fees_for_comp[i] < fee_with_ticket {
      ans += fees_for_comp[i];
    } else {
      ans += fee_with_ticket;
    }
  }

  println!("{}", ans);
}
