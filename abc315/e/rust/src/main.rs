use proconio::*;
use std::collections::HashSet;

fn main() {
  input! {
    n:usize
  }

  let mut graph: Vec<Vec<usize>> = Vec::new();
  for _ in 0..n {
    input! {
      c: usize,
      p: [usize; c]
    }
    // 補正
    graph.push(p.into_iter().map(|x| x - 1).collect());
  }

  let mut done_books: HashSet<usize> = HashSet::new();
  let mut order: Vec<usize> = Vec::new();
  let mut history: Vec<usize> = Vec::new();

  find_basic_book_and_read_route(&1, &graph, &done_books, &mut order);
  read_from_basics(order, &graph, &mut done_books, &mut history);
  println!("{:?}", history);
}

fn find_basic_book_and_read_route(
  target_book: &usize,
  graph: &Vec<Vec<usize>>,
  done_books: &HashSet<usize>,
  order: &mut Vec<usize>,
) {
  order.push(*target_book);
  let nexts = &graph[*target_book];
  for next in nexts {
    if !done_books.contains(&next) {
      find_basic_book_and_read_route(&next, &graph, done_books, order);
      break;
    }
  }
}

fn read_from_basics(
  order: Vec<usize>,
  graph: &Vec<Vec<usize>>,
  done_books: &mut HashSet<usize>,
  history: &mut Vec<usize>,
) {
  // この関数の前提：読むことが可能な本を入力する（この関数でよんでよいかの判定はしない、入力されたものは読んでいいものとする）
  // 本を読んだら、その上位の本を読むこと試みる
  // 前提となる本をすべて読んでいれば、その本を同様に読む
  // 前提となる本で未読のものがあれば、前提となるルートの本を探して同じく読む

  // 本を読む
  for &next in order.iter().rev() {
    let mut ready = true;
    for prerequisit in graph[next].iter() {
      if !done_books.contains(&prerequisit) {
        ready = false;
        break;
      }
    }

    if ready {
      history.push(next + 1);
      done_books.insert(next);
    } else {
      let mut order: Vec<usize> = Vec::new();
      find_basic_book_and_read_route(&next, graph, done_books, &mut order);
      read_from_basics(order, graph, done_books, history);
    }
  }
}
