use std::io::{self};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut a = vec![vec!['0'; n]; n];
    let mut b = vec![vec!['0'; n]; n];

    for i in 0..n {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let row: Vec<char> = input.trim().chars().collect();
        a[i] = row;
    }

    for i in 0..n {
        for j in 0..n {
            b[i][j] = a[i][j];
        }
    }

    // 最上列の処理
    b[0][0] = a[1][0];
    for j in 1..n {
        b[0][j] = a[0][j - 1];
    }
    // 中間列の処理
    for i in 1..n - 1 {
        b[i][0] = a[i + 1][0];
        b[i][n - 1] = a[i - 1][n - 1];
        for j in 1..n - 1 {
            b[i][j] = a[i][j];
        }
    }
    // 最下列の処理
    b[n - 1][n - 1] = a[n - 2][n - 1];
    for j in 0..n - 1 {
        b[n - 1][j] = a[n - 1][j + 1];
    }
    for i in 0..n {
        println!("{}", b[i].iter().collect::<String>());
    }
}
