use io::Write;
use std::{
    collections::{HashMap, VecDeque},
    io, str,
};

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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (m, n) = (scan.token::<usize>(), scan.token::<usize>());
    let mut room = vec![vec![0; n]; m];

    for i in 0..m {
        for j in 0..n {
            room[i][j] = scan.token::<i64>();
        }
    }

    let mut factors_map = HashMap::new();

    for i in 0..m {
        for j in 0..n {
            factors_map
                .entry(room[i][j])
                .or_insert_with(Vec::new)
                .push((i + 1) * (j + 1));
        }
    }

    let mut queue = VecDeque::new();
    let mut visited = vec![false; n * m + 1];

    queue.push_back(n * m);
    visited[n * m] = true;

    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();

        if curr == 1 {
            writeln!(out, "yes").unwrap();
            return;
        }

        if let Some(factors) = factors_map.get(&(curr as i64)) {
            for &factor in factors {
                if !visited[factor] {
                    visited[factor] = true;
                    queue.push_back(factor);
                }
            }
        }
    }

    writeln!(out, "no").unwrap();
}
