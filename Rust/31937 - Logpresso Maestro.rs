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

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut computers_infected = vec![false; n + 1];
    let mut logs = vec![(0, 0, 0); m];

    for _ in 0..k {
        computers_infected[scan.token::<usize>()] = true;
    }

    for i in 0..m {
        logs[i] = (
            scan.token::<i64>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );
    }

    logs.sort();

    for i in 1..=n {
        let mut infected = vec![false; n + 1];
        infected[i] = true;

        for (_, from, to) in logs.iter() {
            if infected[*from] {
                infected[*to] = true;
            }
        }

        if infected == computers_infected {
            writeln!(out, "{i}").unwrap();
            return;
        }
    }
}
