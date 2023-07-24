use permutohedron::LexicalPermutation;
use proconio::input;

fn main() {
  input! {
   s: String,
   mut k: isize
  }

  let mut chars: Vec<char> = s.trim().chars().collect();

  chars.sort_unstable();

  while k > 1 {
    chars.next_permutation();
    k -= 1;
  }

  let s = chars.into_iter().collect::<String>();

  println!("{}", s);
}
