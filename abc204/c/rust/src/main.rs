use std::collections::HashSet;

use proconio::*;

struct Transport {
  destinatios: Vec<usize>,
  terminals: Option<HashSet<usize>>,
}

fn main() {
  input! {
    n: usize,
    m: usize,
    roads: [[usize;2];m]
  }

  let mut transports: Vec<Transport> = Vec::new();
  for _ in 0..n {
    let tran = Transport {
      destinatios: Vec::new(),
      terminals: None,
    };
    transports.push(tran);
  }

  for i in 0..m {
    transports[roads[i][0] - 1]
      .destinatios
      .push(roads[i][1] - 1);
  }

  // find terminals
  for city_id in 0..n {
    let mut set: HashSet<usize> = HashSet::new();
    set = find_terminals(&transports, city_id, &mut set);
    transports[city_id].terminals = Some(set.clone());
    // println!("sourcce: {}, dest: [{:?}]", city_id, set);
  }

  println!(
    "{}",
    transports.into_iter().fold(0, |acc, t| {
      acc
        + match t.terminals {
          Some(d) => d.len(),
          _ => 0,
        }
    })
  );
}

fn find_terminals(
  transports: &Vec<Transport>,
  city_id: usize,
  set: &mut HashSet<usize>,
) -> HashSet<usize> {
  set.insert(city_id);
  for &next_city in &transports[city_id].destinatios {
    if !set.contains(&next_city) {
      match &transports[next_city].terminals {
        Some(next_set) => set.extend(next_set),
        None => {
          set.insert(next_city);
          find_terminals(&transports, next_city, set);
        }
      }
    }
  }
  set.clone()
}
