use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();
    let t = s.parse::<i64>().unwrap();

    for _ in 0..t {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let mut heights = s
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        for height in heights.iter_mut() {
            *height += 1_000_000_000;
        }

        let mut ret = 0;

        for i in 1..heights.len() - 1 {
            let left = *heights[..i].iter().max().unwrap();
            let right = *heights[i + 1..].iter().max().unwrap();

            if heights[i] < left && heights[i] < right {
                ret += std::cmp::min(left, right) - heights[i];
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
