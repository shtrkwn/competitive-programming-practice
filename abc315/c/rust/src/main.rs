use std::collections::HashMap;

use proconio::*;

fn main() {
  input! {
    n:usize,
  }
  let mut ices: HashMap<usize, Vec<usize>> = HashMap::new();
  for _ in 0..n {
    input! {
      flavor: usize,
      satisfaction: usize
    }
    let op_ice = ices.get_mut(&flavor);
    match op_ice {
      Some(vec) => vec.push(satisfaction),
      None => {
        let flavor_vec = ices.entry(flavor).or_insert(Vec::new());
        flavor_vec.push(satisfaction);
      }
    }
  }

  let num_flavors = ices.len();
  let mut maxes: Vec<Vec<usize>> = vec![vec![0, 0]; num_flavors];

  // let mut ices:Vec<Vec<usize>> = Vec::new();
  let mut counter = 0;
  for (_, sats) in ices.iter() {
    let mut sats = sats.clone();
    // 1つのフレーバに１つしかカップがないときにの処理
    if sats.len() == 1 {
      maxes[counter][0] = sats[0];
      maxes[counter][1] = 0;
    } else {
      sats.sort_by(|x1, x2| x2.cmp(x1));
      maxes[counter][0] = sats[0];
      maxes[counter][1] = sats[1];
    }

    counter += 1;
  }

  maxes.sort_by(|x1, x2| x2[0].cmp(&x1[0]));

  let num_flavors = maxes.len();

  // 1種類しかない場合のケア
  let mut max = maxes[0][0] + maxes[0][1] / 2;
  if num_flavors == 1 {
    println!("{}", max);
    return;
  }

  max = max.max(maxes[0][0] + maxes[1][0]);
  for i in 0..num_flavors {
    if maxes[i][0] != 0 {
      max = max.max(maxes[i][0] + maxes[i][1] / 2);
    }
  }
  println!("{:?}", max);
}
