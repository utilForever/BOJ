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
    let mut table = vec![vec![0; n + 1]; n + 1];
    let mut sum_row = vec![vec![0; n + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=n {
            table[i][j] = scan.token::<usize>();
        }
    }

    for i in 1..=n {
        for j in 1..=n {
            sum_row[i][j] = sum_row[i][j - 1] + table[i][j];
        }
    }

    for _ in 1..=m {
        let (x1, y1, x2, y2) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );
        let mut sum = 0;

        for i in x1..=x2 {
            sum += sum_row[i][y2] - sum_row[i][y1 - 1];
        }

        writeln!(out, "{}", sum).unwrap();
    }
}
