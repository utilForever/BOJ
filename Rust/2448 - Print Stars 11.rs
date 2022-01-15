use io::Write;
use std::io;

fn input_integers() -> Vec<i64> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i64> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn draw(map: &mut Vec<Vec<char>>, row: usize, col: usize) {
    map[row][col] = '*';

    map[row + 1][col - 1] = '*';
    map[row + 1][col + 1] = '*';

    for i in 0..5 {
        map[row + 2][col - 2 + i] = '*';
    }
}

fn triangle(map: &mut Vec<Vec<char>>, len: usize, row: usize, col: usize) {
    if len == 3 {
        draw(map, row, col);
        return;
    }

    triangle(map, len / 2, row, col);
    triangle(map, len / 2, row + len / 2, col - len / 2);
    triangle(map, len / 2, row + len / 2, col + len / 2);
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let n = input_integers()[0] as usize;

    let mut map = vec![vec![' '; 6144]; 3072];

    for i in 0..n {
        for j in 0..(2 * n - 1) {
            map[i][j] = ' ';
        }
    }

    triangle(&mut map, n, 0, n - 1);

    for i in 0..n {
        for j in 0..(2 * n - 1) {
            write!(out, "{}", map[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
