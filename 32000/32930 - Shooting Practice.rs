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
    let mut targets_curr = vec![(0, 0); n];
    let mut targets_next = vec![(0, 0); m];

    for i in 0..n {
        targets_curr[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    for i in 0..m {
        targets_next[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let mut pos = (0, 0);
    let mut ret = 0;

    for _ in 0..m {
        targets_curr.sort_by_key(|&(x, y)| (x - pos.0).pow(2) + (y - pos.1).pow(2));
        ret += (targets_curr[n - 1].0 - pos.0).pow(2) + (targets_curr[n - 1].1 - pos.1).pow(2);
        pos = targets_curr[n - 1];

        targets_curr.pop();
        targets_curr.push(targets_next.remove(0));
    }

    writeln!(out, "{ret}").unwrap();
}
