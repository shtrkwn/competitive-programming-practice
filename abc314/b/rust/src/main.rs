use proconio::*;

fn main() {
  input! {
    n: usize,
  }
  let mut a: Vec<Vec<usize>> = Vec::new();
  for _ in 0..n {
    input! {
      c: usize,
      ai: [usize;c]
    }
    a.push(ai);
  }
  input! {
    x:usize
  }

  let mut ans: Vec<(usize, usize)> = a
    .iter()
    .enumerate()
    .filter(|&(_, vec)| vec.contains(&x))
    .map(|(i, vec)| (vec.len(), i))
    .collect();

  ans.sort();

  if ans.is_empty() || ans[0].0 != ans[0].0 {
    println!("0");
    return;
  }

  let values = ans
    .iter()
    .take_while(|&&(len, _)| len == ans[0].0)
    .map(|&(_, i)| i + 1)
    .collect::<Vec<_>>();

  println!("{}", values.len());
  println!(
    "{}",
    values
      .iter()
      .map(|i| i.to_string())
      .collect::<Vec<_>>()
      .join(" ")
  );
}
