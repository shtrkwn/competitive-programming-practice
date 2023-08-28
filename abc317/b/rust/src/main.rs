use proconio::*;
fn main() {
  input! {
    n:usize,
    mut a:[usize;n]
  }

  a.sort();

  // 端の数をなくした場合、一意に定まらないので、端の数は除外する
  // ただし、最小値が２の場合と最大値が１００の場合は、それが端であることが確定するので、
  // その場合は、最小値 or 最大値をなくしている可能性はある

  for i in 0..(n - 1) {
    if a[i + 1] - a[i] != 1 {
      println!("{}", a[i] + 1);
      return;
    }
  }
  if a[0] == 2 {
    println!("{}", 1 + n);
  } else {
    println!("{}", 1000 - n);
  }
}
