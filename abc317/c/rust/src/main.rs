use itertools::Itertools;
use proconio::*;
use std::collections::HashMap;

fn main() {
  input! {
    n:usize,
    m:usize,
    roads:[[usize;3];m]
  }

  // 無向グラフの最長経路長問題
  // ぱっと見難しいが、街がたかだか１０しかないので、全探索で行けそう
  // 全探索するときの最悪計算量は、10!
  // 途中で道が途切れた場合はそこで計算を終了しても問題ない
  // （打ち切られたあとで出現する部分経路が先頭になるようなpermが存在するのでいずれ計算されるため）
  // 逆に道路の検索に時間かかると怪しいの、O(1)で検索できるデータの持ち方をする

  // 道路をHashMapで持つ
  let mut road_map: HashMap<(usize, usize), usize> = HashMap::new();
  for road in roads.iter() {
    road_map.insert((road[0] - 1, road[1] - 1), road[2]);
    road_map.insert((road[1] - 1, road[0] - 1), road[2]);
  }

  // 全探索
  let cities: Vec<usize> = (0..n).collect();
  let mut road_len = 0;
  for perm in cities.iter().permutations(cities.len()) {
    let mut road_len_tmp = 0;
    for i in 0..(n - 1) {
      match road_map.get(&(*perm[i], *perm[i + 1])) {
        Some(len) => road_len_tmp += len,
        None => break,
      }
    }
    road_len = road_len.max(road_len_tmp);
  }
  println!("{}", road_len);
}
