use proconio::*;

fn main() {
  input! {
    n:usize,
    p: [[usize;2];n]
  }

  let mut p_sorted_by_x = p.clone();
  let mut p_sorted_by_y = p.clone();

  p_sorted_by_x.sort_by(|a, b| {
    if a[0] != b[0] {
      a[0].cmp(&b[0])
    } else {
      a[1].cmp(&b[1])
    }
  });

  p_sorted_by_y.sort_by(|a, b| {
    if a[1] != b[1] {
      a[1].cmp(&b[1])
    } else {
      a[0].cmp(&b[0])
    }
  });

  let x: Vec<usize> = p_sorted_by_x.iter().map(|pi| pi[0]).collect();
  let y: Vec<usize> = p_sorted_by_y.iter().map(|pi| pi[1]).collect();

  let mut ans = 0;
  for i in 0..(n - 3) {
    // set p1 (x1, y1)
    let x1 = p_sorted_by_x[i][0];
    let y1 = p_sorted_by_x[i][1];

    // try to find p2 (x1, y2)
    for j in (i + 1)..n {
      if p_sorted_by_x[j][0] != x1 {
        break;
      }
      // found p2 candidate
      let y2 = p_sorted_by_x[j][1];

      // try to find p3 (x2, y1)
      let k = find_first_match(&y, y1).ok().unwrap();
      for k in k..n {
        if p_sorted_by_y[k][1] != y1 {
          break;
        }
        if p_sorted_by_y[k][0] <= x1 {
          continue;
        }
        // found p3 (x2, y1) candidate
        let x2 = p_sorted_by_y[k][0];

        // try to find p4 (x2, y2)
        let l = find_first_match(&x, x2).ok().unwrap();
        for l in l..n {
          if p_sorted_by_x[l][0] != x2 {
            break;
          }
          if p_sorted_by_x[l][1] == y2 {
            // found p4
            // this means there exist rect angle satsfying the condition
            ans += 1;
            break;
          }
        }
      }
    }
  }

  println!("{}", ans);
}

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
