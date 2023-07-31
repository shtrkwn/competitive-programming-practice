use proconio::input;

fn main() {
  input! {
    n:usize,
    m:usize,
    s:[String;n]
  }
  let grid: Vec<Vec<char>> = s.into_iter().map(|s| s.chars().collect()).collect();

  for row in 0..(n - 8) {
    for col in 0..(m - 8) {
      let mut result = true;

      // 黒いとこの判定
      for r in 0..3 {
        for c in 0..3 {
          result = result && grid[row + r][col + c] == '#';
          result = result && grid[row + r + 6][col + c + 6] == '#';
        }
      }

      // 白いとこの判定
      for c in 0..4 {
        result = result && grid[row + 3][col + c] == '.';
        result = result && grid[row + 5][col + 8 - c] == '.';
      }
      for r in 0..4 {
        result = result && grid[row + r][col + 3] == '.';
        result = result && grid[row + 8 - r][col + 5] == '.';
      }

      if result {
        println!("{} {}", row + 1, col + 1);
      }
    }
  }
}
