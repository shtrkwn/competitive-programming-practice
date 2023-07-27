use proconio::input;

fn main() {
  input! {
  n: usize
  }

  let n_array: Vec<usize> = format!("{}", n)
    .chars()
    .map(|c| c.to_string().parse().unwrap())
    .collect();

  let mut max: u64 = 0;
  for i in 0..2_i32.pow(n_array.len() as u32) {
    let i_bin: Vec<char> = format!("{:b}", i).chars().collect();

    // 桁が足りない場合、比較するまでもなく最大値ではないので除外
    if i_bin.len() != n_array.len() {
      continue;
    }
    let mut first_group: Vec<usize> = n_array
      .iter()
      .enumerate()
      .filter(|&(i, _)| i_bin[i] == '0')
      .map(|(_, e)| *e)
      .collect();
    let mut second_group: Vec<usize> = n_array
      .iter()
      .enumerate()
      .filter(|&(i, _)| i_bin[i] == '1')
      .map(|(_, e)| *e)
      .collect();

    if first_group.is_empty() || second_group.is_empty() {
      continue;
    }

    first_group.sort_by(|a, b| b.cmp(a));
    let first_num: String = first_group.into_iter().map(|n| n.to_string()).collect();
    let first_num: usize = first_num.parse().unwrap();

    second_group.sort_by(|a, b| b.cmp(a));
    let second_num: String = second_group.into_iter().map(|n| n.to_string()).collect();
    let second_num: usize = second_num.parse().unwrap();

    let mul = first_num as u64 * second_num as u64;
    if max < mul {
      max = mul
    }
  }

  println!("{}", max);
}
