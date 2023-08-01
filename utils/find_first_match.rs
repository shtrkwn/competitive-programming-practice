// ２分探索を行い、条件にマッチする要素が配列に複数ある場合は、先頭の要素のインデックスを返却する関数

fn find_first_match(vec: &[usize], target: usize) -> Result<usize, usize> {
  let mut low = 0;
  let mut high = vec.len();

  while low < high {
    let mid = (low + high) / 2;
    if vec[mid] < target {
      low = mid + 1;
    } else {
      high = mid;
    }
  }

  if low < vec.len() && vec[low] == target {
    Ok(low)
  } else {
    Err(low)
  }
}
