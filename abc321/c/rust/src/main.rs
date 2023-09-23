use proconio::*;

fn main() {
  input! {k:usize}

  let mut tto_like: Vec<u64> = vec![];

  for i in (1..10).rev() {
    tto_like.push(i);
    tto_like.extend(gen_nexts(i));
  }

  tto_like.sort();

  println!("{}", tto_like[(k - 1)]);
}

fn gen_nexts(n: u64) -> Vec<u64> {
  let mut nexts: Vec<u64> = Vec::new();
  let last_digit = n % 10;

  for i in (0..last_digit).rev() {
    let next = n * 10 + i;
    nexts.extend(gen_nexts(next));
    nexts.push(n * 10 + i);
  }

  return nexts;
}
