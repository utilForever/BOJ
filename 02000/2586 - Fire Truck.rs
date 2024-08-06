use io::Write;
use std::{io, str};

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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum ObjectType {
    Pump,
    FireTruck,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (p, f) = (scan.token::<usize>(), scan.token::<usize>());
    let mut road = vec![(0, ObjectType::Pump); p + f];

    for i in 0..p {
        road[i] = (scan.token::<i64>(), ObjectType::Pump);
    }

    for i in 0..f {
        road[p + i] = (scan.token::<i64>(), ObjectType::FireTruck);
    }

    road.sort();
    road.insert(0, (0, ObjectType::Pump));

    let mut depth = p + f;
    let mut tree = vec![Vec::new(); (p + f + 1) * 2];

    for i in 1..=p + f {
        match road[i].1 {
            ObjectType::Pump => {
                tree[depth].push(road[i].clone());
                depth += 1;
            }
            ObjectType::FireTruck => {
                depth -= 1;
                tree[depth].push(road[i].clone());
            }
        }
    }

    let mut total_dist = 0;

    for i in 0..(p + f + 1) * 2 {
        if tree[i].is_empty() {
            continue;
        }

        let mut dist = 0;

        for j in (1..tree[i].len()).step_by(2) {
            dist += (tree[i][j].0 - tree[i][j - 1].0).abs();
        }

        if tree[i].len() % 2 == 0 {
            total_dist += dist;
            continue;
        }

        let mut min_dist = dist;

        for j in (2..tree[i].len()).rev().step_by(2) {
            dist += (tree[i][j].0 - tree[i][j - 1].0).abs()
                - (tree[i][j - 1].0 - tree[i][j - 2].0).abs();
            min_dist = min_dist.min(dist);
        }

        total_dist += min_dist;
    }

    writeln!(out, "{}", total_dist).unwrap();
}
