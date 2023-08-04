use proconio::*;

fn main() {
  input! {
    n:u64
  }

  const DIV: u64 = 998244353;
  let order: u32 = n.to_string().len() as u32 - 1;

  let mut ans: u64;
  if n > 9 {
    ans = ((n - (10_u64.pow(order) - 1)) % DIV) * ((n - (10_u64.pow(order) - 2)) % DIV) / 2;
    ans %= DIV;

    for o in 0..order {
      let tmp = 9 * 10_u64.pow(o) % DIV;
      ans += tmp * (tmp + 1) / 2;
      ans %= DIV;
    }
  } else {
    ans = n * (n + 1) / 2
  }
  println!("{}", ans);
}
