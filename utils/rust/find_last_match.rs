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
