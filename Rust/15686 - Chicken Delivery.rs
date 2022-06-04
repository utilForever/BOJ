use io::Write;
use std::{cmp, io, str};

pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}

fn calculate_chicken_distance(
    houses: &Vec<(i64, i64)>,
    chickens: &Vec<(i64, i64)>,
    m: usize,
    visited: &mut Vec<usize>,
    ret: &mut i64,
    idx: usize,
    cnt: usize,
) {
    if idx > chickens.len() {
        return;
    }

    if cnt == m {
        let mut dist_total = 0;

        for i in 0..houses.len() {
            let mut dist = 1_000_000_007;

            for j in 0..visited.len() {
                dist = cmp::min(
                    dist,
                    (houses[i].0 - chickens[visited[j]].0).abs()
                        + (houses[i].1 - chickens[visited[j]].1).abs(),
                );
            }

            dist_total += dist;
        }

        if *ret > dist_total {
            *ret = dist_total;
        }

        return;
    }

    visited.push(idx);

    calculate_chicken_distance(houses, chickens, m, visited, ret, idx + 1, cnt + 1);

    visited.pop();

    calculate_chicken_distance(houses, chickens, m, visited, ret, idx + 1, cnt);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut houses = Vec::new();
    let mut chickens = Vec::new();

    for i in 1..=n {
        for j in 1..=n {
            let info = scan.token::<usize>();

            match info {
                0 => (),
                1 => houses.push((i as i64, j as i64)),
                2 => chickens.push((i as i64, j as i64)),
                _ => panic!("Invalid info"),
            }
        }
    }

    let mut visited = Vec::new();
    let mut ret = 1_000_000_007;

    calculate_chicken_distance(&houses, &chickens, m, &mut visited, &mut ret, 0, 0);

    writeln!(out, "{}", ret).unwrap();
}
