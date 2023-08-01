use proconio::*;

fn main() {
  input! {
    n: usize,
    m:usize,
    mut a: [usize;n],
    mut b: [usize;m]
  }

  let mut bp1: Vec<usize> = b.iter().map(|v| v + 1).collect();
  let mut a_and_b: Vec<usize> = Vec::new();

  a.sort();
  b.sort();
  bp1.sort();

  a_and_b.extend(a.clone());
  a_and_b.extend(bp1.clone());

  a_and_b.sort();

  for price in a_and_b {
    // num_a = その価格で売れる人の数
    let num_a = match find_first_match(&a, price + 1) {
      Ok(i) => i,
      Err(i) => i,
    } as isize;

    // num_b = その価格で買える人の数
    //       = 買人の数 - その価格では買えない人の数
    let num_b = m as isize
      - match find_first_match(&b, price) {
        Ok(i) => i as isize,
        Err(i) => i as isize,
      };

    // println!("price: {}, num_a: {}, num_b: {}", price, num_a, num_b);

    if num_a >= num_b {
      println!("{}", price);
      return;
    }
  }
  println!("{}", bp1.last().unwrap());
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
