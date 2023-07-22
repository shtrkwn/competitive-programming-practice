use proconio::input;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Location {
  x: usize,
  y: usize,
}

fn main() {
  input! {
      n: usize,
      _: usize,
      s: [String;n]
  }

  let map: Vec<Vec<char>> = s.iter().map(|s| s.chars().collect()).collect();

  let mut histories: HashSet<Location> = HashSet::new();
  search_new_location(Location { x: 1, y: 1 }, &mut histories, &map);

  println!("{:?}", histories);
}

fn search_new_location(loc: Location, histories: &mut HashSet<Location>, map: &Vec<Vec<char>>) {
  if histories.contains(&loc) {
    return;
  }

  histories.insert(loc.clone());

  let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
  for (dx, dy) in directions.iter() {
    let mut x = loc.x;
    let mut y = loc.y;

    loop {
      x = (x as isize + dx) as usize;
      y = (y as isize + dy) as usize;

      if x < map.len()
        && y < map[0].len()
        && map[y][x] != '#'
        && !histories.contains(&Location { x, y })
      {
        search_new_location(Location { x, y }, histories, map);
      } else {
        break;
      }
    }
  }
}
