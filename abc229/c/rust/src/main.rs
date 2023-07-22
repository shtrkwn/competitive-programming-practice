use proconio::input;

fn main() {
  input! {
    n: usize,
    w: u64,
    mut ab: [(u64, u64);n],
  };

  ab.sort_by(|abi, abj| abj.0.cmp(&abi.0));

  let mut rem: u64 = w;
  let mut val: u64 = 0;
  for i in 0..n {
    if rem > ab[i].1 {
      rem -= ab[i].1;
      val += ab[i].0 * ab[i].1;
    } else {
      val += ab[i].0 * rem;
      break;
    };
  }
  println!("{}", val);
}
