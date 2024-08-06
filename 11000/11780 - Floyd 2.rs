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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut cost = vec![vec![0; n + 1]; n + 1];
    let mut tracking = vec![vec![Vec::new(); n + 1]; n + 1];

    for _ in 0..m {
        let (a, b, c) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        if cost[a][b] != 0 {
            cost[a][b] = cost[a][b].min(c);
        } else {
            cost[a][b] = c;
        }
    }

    for k in 1..=n {
        for i in 1..=n {
            for j in 1..=n {
                if i == j {
                    continue;
                }

                if cost[i][k] == 0 || cost[k][j] == 0 {
                    continue;
                }

                if cost[i][j] == 0 || cost[i][j] > cost[i][k] + cost[k][j] {
                    cost[i][j] = cost[i][k] + cost[k][j];

                    tracking[i][j] = tracking[i][k].clone();
                    tracking[i][j].push(k);

                    let mut rest = tracking[k][j].clone();
                    tracking[i][j].append(&mut rest);
                }
            }
        }
    }

    for i in 1..=n {
        for j in 1..=n {
            write!(out, "{} ", cost[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }

    for i in 1..=n {
        for j in 1..=n {
            if cost[i][j] == 0 {
                writeln!(out, "0").unwrap();
                continue;
            }

            let mut path = tracking[i][j].clone();
            path.insert(0, i);
            path.push(j);

            write!(out, "{} ", path.len()).unwrap();

            for p in path {
                write!(out, "{p} ").unwrap();
            }

            writeln!(out).unwrap();
        }
    }
}
