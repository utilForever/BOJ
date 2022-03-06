use std::{cmp, io};

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

fn main() {
    let n = input_integers()[0] as usize;
    let mut vertices = vec![(0, 0); n];

    for i in 0..n {
        let values = input_integers();
        vertices[i] = (values[0] + 500000, values[1] + 500000);
    }

    let mut x_count = vec![0; 1_000_001];
    let mut y_count = vec![0; 1_000_001];

    for i in 0..n {
        let (x, y) = vertices[i];
        let (x_next, y_next) = vertices[(i + 1) % n];

        if x == x_next {
            let y_max = cmp::max(y, y_next) as usize;
            let y_min = cmp::min(y, y_next) as usize;

            y_count[y_min] += 1;
            y_count[y_max] -= 1;
        } else {
            let x_max = cmp::max(x, x_next) as usize;
            let x_min = cmp::min(x, x_next) as usize;

            x_count[x_min] += 1;
            x_count[x_max] -= 1;
        }
    }

    for i in 1..1_000_001 {
        x_count[i] += x_count[i - 1];
        y_count[i] += y_count[i - 1];
    }

    let mut ans = 0;
    for i in 1..1_000_001 {
        ans = cmp::max(cmp::max(x_count[i], y_count[i]), ans);
    }

    println!("{}", ans);
}
