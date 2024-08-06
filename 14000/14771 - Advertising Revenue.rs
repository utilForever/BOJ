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

    let k = scan.token::<i64>();

    for i in 1..=k {
        let (n, v) = (scan.token::<usize>(), scan.token::<i64>());
        let mut advertisements = vec![(0, 0); n];
        let mut ret = 0;

        for j in 0..n {
            advertisements[j] = (scan.token::<i64>(), scan.token::<i64>());
        }

        for _ in 0..v {
            let (a1, a2, c) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i64>(),
            );

            if advertisements[a1 - 1].0 == 1 || (advertisements[a1 - 1].0 == 0 && c == 1) {
                ret += advertisements[a1 - 1].1;
            }

            if advertisements[a2 - 1].0 == 1 || (advertisements[a2 - 1].0 == 0 && c == 2) {
                ret += advertisements[a2 - 1].1;
            }
        }

        writeln!(out, "Data Set {i}:").unwrap();
        writeln!(out, "{ret}").unwrap();

        if i != k {
            writeln!(out).unwrap();
        }
    }
}
