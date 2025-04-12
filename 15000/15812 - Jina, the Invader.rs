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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut map = vec![vec![' '; m]; n];

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            map[i][j] = c;
        }
    }

    let mut towns = Vec::new();
    let mut empties = Vec::new();

    for i in 0..n {
        for j in 0..m {
            if map[i][j] == '1' {
                towns.push((i, j));
            } else {
                empties.push((i, j));
            }
        }
    }

    let mut ret = i64::MAX;

    for i in 0..empties.len() {
        for j in i + 1..empties.len() {
            let mut dist_max = 0;

            for k in 0..towns.len() {
                let dist1 = (towns[k].0 as i64 - empties[i].0 as i64).abs()
                    + (towns[k].1 as i64 - empties[i].1 as i64).abs();
                let dist2 = (towns[k].0 as i64 - empties[j].0 as i64).abs()
                    + (towns[k].1 as i64 - empties[j].1 as i64).abs();

                dist_max = dist_max.max(dist1.min(dist2));
            }

            ret = ret.min(dist_max);
        }
    }

    writeln!(out, "{ret}").unwrap();
}
