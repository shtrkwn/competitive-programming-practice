use proconio::*;

fn main() {
  input! {
   n:usize,
   x:usize,
   mut a: [usize;n-1]
  }

  a.sort();

  let mut current_score: usize = a.iter().sum();
  current_score = current_score - a.first().unwrap() - a.last().unwrap();
  let maximum_score = current_score + a.last().unwrap();
  let minimum_score = current_score + a.first().unwrap();

  //  どのような点数をとってもXに到達しない場合
  if x > maximum_score {
    println!("-1");
    return;
  }

  // どのような点数をとってもXに到達する場合（最低点を下回っても
  if x <= minimum_score {
    println!("0");
    return;
  }

  // Nラウンドの点数次第の場合
  let point = x - current_score;
  println!("{}", point);
}
