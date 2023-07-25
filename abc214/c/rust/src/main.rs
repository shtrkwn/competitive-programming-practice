use proconio::input;

fn main() {
  input! {
    n: usize,
    s: [usize;n],
    t:[usize;n]
  }

  let mut prop_time: Vec<usize> = vec![0; n];
  prop_time[0] = t[0].min(t[n - 1] + s[n - 1]);

  for i in 1..n {
    prop_time[i] = t[i].min(prop_time[i - 1] + s[i - 1]);
  }

  // 2回目
  prop_time[0] = t[0].min(prop_time[n - 1] + s[n - 1]);
  for i in 1..n {
    prop_time[i] = t[i].min(prop_time[i - 1] + s[i - 1]);
  }

  prop_time.iter().for_each(|time| {
    println!("{}", time);
  });
}
