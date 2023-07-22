use proconio::input;

fn main() {
  input! {
      n: usize,
      mut a: [usize; n],
  }

  let a: Vec<usize> = a.iter().map(|v| v - 1).collect();

  let mut cursor = 0;
  for _ in 0..(n) {
    cursor = a[cursor];
  }

  let mut l: Vec<usize> = vec![cursor];
  for _ in 0..(n) {
    l.push(a[cursor]);
    cursor = a[cursor];

    if cursor == l[0] {
      break;
    }
  }

  let mut ans: String = String::from("");
  for i in 0..(l.len() - 1) {
    ans.push_str(&(l[i] + 1).to_string());
    ans.push_str(" ");
  }
  println!("{}", l.len() - 1);
  println!("{}", ans.trim());
  return;
}
