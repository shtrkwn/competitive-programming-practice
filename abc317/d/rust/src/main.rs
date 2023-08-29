use proconio::*;

fn main() {
  input! {
      n: usize,
      constituencies: [[usize; 3]; n]
  }

  // sum:  全議席数
  let num_all_seats: usize = constituencies.iter().map(|x| x[2]).sum();

  // 高橋くんの支持者数
  let seats_for_takahashi: usize = constituencies
    .iter()
    .filter(|con| con[0] > con[1])
    .map(|x| x[2])
    .sum();

  // すでに高橋くんが過半数の議席を取得している場合、選挙活動終了
  if seats_for_takahashi >= num_all_seats / 2 + 1 {
    println!("0");
    return;
  }

  // 追加獲得が必要な議席数
  let required_additional_seats = num_all_seats / 2 + 1 - seats_for_takahashi;

  // ナップザック問題として解決する
  // dp[i] := i議席を追加獲得するために必要最小限の支持変更者数
  // 初期値：全有権者の指示を得た場合の各得票数（これより多くなることはない）
  let mut dp = vec![1e18 as u64; num_all_seats + 1];
  dp[0] = 0;

  for constituency in constituencies {
    // すでに高橋くんが勝利している選挙区では、支持者を増やす必要はない
    if constituency[0] > constituency[1] {
      continue;
    }

    // 寝返らせる支持者の数
    let required_efforts = ((constituency[1] - constituency[0]) / 2 + 1) as u64;

    // この選挙区で勝利することで獲得できる議席の数
    let num_seats = constituency[2];

    // dp[i]の多重更新回避のために一時配列
    let mut dp_tmp = dp.clone();
    for i in 0..(dp.len() - num_seats) {
      // 変更先のインデックス
      let j = i + num_seats;
      if dp[j] > dp[i] + required_efforts {
        dp_tmp[j] = dp[i] + required_efforts;
      }
    }
    dp = dp_tmp;
  }

  let min_efforts = dp[required_additional_seats..dp.len()].iter().min();
  let ans = match min_efforts {
    Some(num) => num,
    None => &0_u64,
  };

  println!("{}", ans);
}
