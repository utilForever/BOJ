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

    let (m, n) = (scan.token::<usize>(), scan.token::<usize>());
    let mut windows = vec![vec![' '; 5 * n + 1]; 5 * m + 1];

    for i in 0..5 * m + 1 {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            windows[i][j] = c;
        }
    }

    let mut ret = [0; 5];

    for i in 0..m {
        for j in 0..n {
            let idx = 5 * i + 1;
            let mut cnt = 0;

            for k in 0..4 {
                if windows[idx + k][5 * j + 2] == '*' {
                    cnt += 1;
                }
            }

            ret[cnt] += 1;
        }
    }

    for i in 0..5 {
        write!(out, "{} ", ret[i]).unwrap();
    }

    writeln!(out).unwrap();
}
