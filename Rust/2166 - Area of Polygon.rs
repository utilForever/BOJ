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

fn calculate_ccw(p1: (i64, i64), p2: (i64, i64), p3: (i64, i64)) -> i64 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let (x3, y3) = p3;

    (x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1)
}

fn main() {
    let n = input_integers()[0] as usize;
    let mut vertices = vec![(0, 0); n];

    for i in 0..n {
        let nums = input_integers();
        vertices[i] = (nums[0], nums[1]);
    }

    let mut area: f64 = 0.0;

    for i in 1..(n - 1) {
        area += calculate_ccw(vertices[0], vertices[i], vertices[i + 1]) as f64 / 2.0;
    }

    println!("{:.1}", area.abs());
}
