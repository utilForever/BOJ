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
    let mut nails = vec![vec![0; n + 3]; n + 3];

    for _ in 0..m {
        let (a, b, x) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        nails[a][b] += 1;
        nails[a][b + 1] -= 1;
        nails[a + x + 1][b] -= 1;
        nails[a + x + 1][b + x + 2] += 1;
        nails[a + x + 2][b + 1] += 1;
        nails[a + x + 2][b + x + 2] -= 1;
    }

    // Sweeping from left to right
    for i in 1..n + 3 {
        for j in 1..n + 3 {
            nails[i][j] += nails[i][j - 1];
        }
    }

    // Sweeping from top to bottom
    for i in 1..n + 3 {
        for j in 1..n + 3 {
            nails[i][j] += nails[i - 1][j];
        }
    }

    // Sweeping diagonal
    for i in 1..n + 3 {
        for j in 1..n + 3 {
            nails[i][j] += nails[i - 1][j - 1];
        }
    }

    let mut ret = 0;

    for i in 1..n + 3 {
        for j in 1..n + 3 {
            if nails[i][j] > 0 {
                ret += 1;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
