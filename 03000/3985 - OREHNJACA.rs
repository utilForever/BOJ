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

    let (l, n) = (scan.token::<usize>(), scan.token::<i64>());
    let mut rollcake = vec![0; l + 1];
    let mut max_expected = (0, 0);
    let mut max_real = (0, 0);

    for i in 1..=n {
        let (p, k) = (scan.token::<usize>(), scan.token::<usize>());
        let expected = k - p + 1;

        if expected > max_expected.1 {
            max_expected = (i, expected);
        }

        let mut cnt = 0;

        for j in p..=k {
            if rollcake[j] == 0 {
                rollcake[j] = i;
                cnt += 1;
            }
        }

        if cnt > max_real.1 {
            max_real = (i, cnt);
        }
    }

    writeln!(out, "{}", max_expected.0).unwrap();
    writeln!(out, "{}", max_real.0).unwrap();
}
