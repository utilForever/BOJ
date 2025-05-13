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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, c) = (scan.token::<usize>(), scan.token::<i64>());
    let mut fishes = vec![(0, 0, 0); n];

    for i in 0..n {
        fishes[i] = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
    }

    fishes.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    let bait_sum = fishes.iter().map(|x| x.0).sum::<i64>();
    let mut ret = 0;

    for bait in 0..=bait_sum {
        let mut bait_remain = bait;
        let mut visited = vec![false; n];
        let mut price_total = 0;

        loop {
            let mut check = false;

            for i in 0..fishes.len() {
                if visited[i] {
                    continue;
                }

                if fishes[i].0 <= bait_remain {
                    bait_remain -= fishes[i].0;
                    visited[i] = true;
                    price_total += fishes[i].2;
                    check = true;
                    break;
                }
            }

            if !check {
                break;
            }
        }

        let profit = price_total - bait * c;
        ret = ret.max(profit);
    }

    writeln!(out, "{ret}").unwrap();
}
