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
    let mut land = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            land[i][j] = scan.token::<i32>();
        }
    }

    let mut cnt_cardinal = 0;
    let mut cnt_intercardinal = 0;

    for i in 0..n {
        for j in 0..m {
            if land[i][j] == 1 && j < m - 1 && land[i][j + 1] == 1 {
                cnt_cardinal += 1;
                cnt_intercardinal += 1;
            }

            if land[i][j] == 1 && i < n - 1 && land[i + 1][j] == 1 {
                cnt_cardinal += 1;
                cnt_intercardinal += 1;
            }

            if land[i][j] == 1 && i < n - 1 && j < m - 1 && land[i + 1][j + 1] == 1 {
                cnt_intercardinal += 1;
            }

            if land[i][j] == 1 && i < n - 1 && j > 0 && land[i + 1][j - 1] == 1 {
                cnt_intercardinal += 1;
            }
        }
    }

    writeln!(out, "{cnt_cardinal} {cnt_intercardinal}").unwrap();
}
