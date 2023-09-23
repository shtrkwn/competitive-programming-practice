use proconio::*;

fn main() {
  input! {
    n: usize,
    m: usize,
    p: usize,
    mut a: [usize; n],
    mut b: [usize; m]
  }

  a.sort();
  b.sort();

  // v[i] := 安い順に副菜を並べたときのiまでの総和
  let mut side_menu_price_sum: Vec<usize> = vec![];
  for sp in &b {
    side_menu_price_sum.push(side_menu_price_sum.last().unwrap_or(&0) + sp);
  }

  let mut ans = 0;
  for mp in a {
    if p < mp {
      ans += p * m;
      continue;
    }
    let max = p - mp;

    let i: isize = match find_last_match(&b, max) {
      Ok(index) => index as isize,
      Err(index) => index as isize - 1,
    };

    if i < 0 {
      ans += p * m;
      continue;
    }

    let i = i as usize;
    ans += mp * (i + 1) + side_menu_price_sum[i];

    // pを超える部分の処理
    ans += p * (m - (i + 1));
  }
  println!("{}", ans);
}

// ２分探索を行い、条件にマッチする要素が配列に複数ある場合は、末尾の要素のインデックスを返却する関数
fn find_last_match(vec: &[usize], target: usize) -> Result<usize, usize> {
  let mut low = 0;
  let mut high = vec.len();

  while low < high {
    let mid = (low + high) / 2;
    if vec[mid] <= target {
      low = mid + 1;
    } else {
      high = mid;
    }
  }

  if low > 0 && vec[low - 1] == target {
    Ok(low - 1)
  } else {
    Err(low)
  }
}
