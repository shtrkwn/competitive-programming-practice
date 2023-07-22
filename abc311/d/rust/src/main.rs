use proconio::input;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Location {
  x: usize,
  y: usize,
}

#[derive(Debug)]
struct Map {
  state: char,
  is_passed: bool,
}

fn main() {
  input! {
      n: usize,
      _: usize,
      s: [String;n]
  }

  let mut map: Vec<Vec<Map>> = s
    .iter()
    .map(|s| {
      s.chars()
        .map(|c| Map {
          state: c,
          is_passed: false,
        })
        .collect()
    })
    .collect();

  let mut histories: HashSet<Location> = HashSet::new();
  search_new_location(Location { x: 1, y: 1 }, &mut histories, &mut map);
  println!(
    "{}",
    map.iter().fold(0, |acc, row| {
      acc
        + row
          .iter()
          .fold(0, |acc, m| if m.is_passed { acc + 1 } else { acc })
    })
  )
}

fn search_new_location(loc: Location, histories: &mut HashSet<Location>, map: &mut Vec<Vec<Map>>) {
  if histories.contains(&loc) {
    return;
  }

  histories.insert(loc.clone());

  let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
  for (dx, dy) in directions.iter() {
    let mut mul_fact = 0;
    loop {
      let x = (loc.x as isize + dx * mul_fact) as usize;
      let y = (loc.y as isize + dy * mul_fact) as usize;

      if map[y][x].state == '#' {
        let x = (x as isize - dx) as usize;
        let y = (y as isize - dy) as usize;
        if !histories.contains(&Location { x, y }) {
          search_new_location(Location { x, y }, histories, map);
        }
        break;
      } else {
        map[y][x].is_passed = true;
      }
      mul_fact += 1;
    }
  }
}
