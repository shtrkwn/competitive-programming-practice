use proconio::*;

fn main() {
  input! {
    n:usize,
    coordinates: [[usize;4];n]
  }

  let grid_len = 102;
  let mut grid: Vec<Vec<isize>> = vec![vec![0; grid_len]; grid_len];

  for coordinate in coordinates {
    let x_min = coordinate[0].min(coordinate[1]);
    let x_max = coordinate[0].max(coordinate[1]);
    let y_min = coordinate[2].min(coordinate[3]);
    let y_max = coordinate[2].max(coordinate[3]);

    grid[y_min][x_min] += 1;
    grid[y_max][x_max] += 1;
    grid[y_min][x_max] -= 1;
    grid[y_max][x_min] -= 1;
  }
  for y in 0..grid_len {
    for x in 0..grid_len {
      if y > 0 {
        grid[y][x] += grid[y - 1][x];
      }
      if x > 0 {
        grid[y][x] += grid[y][x - 1];
      }
      if y > 0 && x > 0 {
        grid[y][x] -= grid[y - 1][x - 1];
      }
    }
  }

  let mut sum = 0;
  for y in 0..grid_len - 1 {
    for x in 0..grid_len - 1 {
      if grid[y][x] > 0 {
        sum += 1;
      }
    }
  }
  println!("{}", sum);
}
