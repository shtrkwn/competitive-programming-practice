use proconio::input;

struct Pos {
  xy: Vec<usize>,
  index: usize,
}

fn main() {
  input! {
    _: usize,
    _: usize,
    n: usize,
    ab: [[usize;2];n]
  };

  let mut ans: Vec<Pos> = ab
    .into_iter()
    .enumerate()
    .map(|(i, vec)| Pos {
      index: i,
      xy: vec![vec[0], vec[1]],
    })
    .collect();

  for wh in 0..=1 {
    ans.sort_by(|x1, x2| x1.xy[wh].cmp(&x2.xy[wh]));

    let mut max = ans[0].xy[wh];
    let mut index: usize = 1;
    ans = ans
      .into_iter()
      .map(|p| {
        if max < p.xy[wh] {
          max = p.xy[wh];
          index += 1;
        }
        return Pos {
          xy: if wh == 0 {
            vec![index, p.xy[1]]
          } else {
            vec![p.xy[0], index]
          },
          index: p.index,
        };
      })
      .collect();
  }

  ans.sort_by(|x1, x2| x1.index.cmp(&x2.index));

  ans.iter().for_each(|p| {
    println!("{} {}", p.xy[0], p.xy[1]);
  });
}
