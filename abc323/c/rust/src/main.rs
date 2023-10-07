use proconio::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    a:[usize;m],
    s:[String;n]
  }

  let mut current_scores: Vec<(usize, Vec<usize>)> = vec![];

  for i in 0..n {
    let mut score = 0;
    let mut unsolved: Vec<usize> = vec![];

    let current: Vec<char> = s[i].chars().collect();
    for j in 0..m {
      if current[j] == 'o' {
        score += a[j];
      } else if current[j] == 'x' {
        unsolved.push(a[j]);
      }
    }

    unsolved.sort_by(|x1, x2| x2.cmp(x1));
    let mut steps: Vec<usize> = vec![];
    for u in unsolved {
      if steps.is_empty() {
        steps.push(u);
      } else {
        steps.push(steps.last().unwrap() + u);
      }
    }
    current_scores.push((score + i + 1, steps));
  }

  let high_score: usize = *current_scores.iter().map(|(num, _)| num).max().unwrap();

  for i in 0..n {
    if current_scores[i].0 != high_score {
      for j in 0..m {
        if high_score < current_scores[i].0 + current_scores[i].1[j] {
          println!("{}", j + 1);
          break;
        }
      }
    } else {
      println!("0");
    }
  }
}
