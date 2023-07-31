use proconio::input;

fn main() {
  input! {
      s: String
  }

  let array: [&str; 7] = ["ACE", "BDF", "CEG", "DFA", "EGB", "FAC", "GBD"];

  if array.contains(&s.as_str()) {
    println!("Yes");
  } else {
    println!("No");
  }
}
