use proconio::*;

fn main() {
  input! {
    n: u64
  }

  let mut ans = 0_u64;
  let mut a = 1_u64;
  loop {
    if a * a * a <= n {
      // aを固定してbを計算
      let mut b = a;
      loop {
        if b * b * a <= n {
          // bを固定してcの最大値を計算
          let c_max = n / a / b;

          // b <= c <= c_maxなので cの取りうる値は c_max - b + 1 通り
          ans += c_max - b + 1;
          b += 1;
        } else {
          break;
        }
      }
      a += 1;
    } else {
      break;
    }
  }

  println!("{}", ans);
}
