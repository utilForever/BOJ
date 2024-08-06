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

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let n = input_integers()[0] as usize;

    let mut points = vec![(0, 0); n as usize];

    for i in 0..n {
        let nums = input_integers();
        let x = nums[0];
        let y = nums[1];

        points[i] = (x, y);
    }

    points.sort_by(|a, b| {
        if a.1 == b.1 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });

    for point in points.iter() {
        writeln!(out, "{} {}", point.0, point.1).unwrap();
    }
}
