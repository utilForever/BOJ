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

    let n = scan.token::<usize>();
    let mut matrix = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            matrix[i][j] = scan.token::<i64>();
        }
    }

    let (r, c) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);

    let mut dp1 = vec![vec![i64::MIN; n]; n];
    dp1[0][0] = matrix[0][0];

    for i in 0..n {
        for j in 0..n {
            if i == 0 && j == 0 {
                continue;
            }

            let mut val_max = i64::MIN;

            if i > 0 {
                val_max = val_max.max(dp1[i - 1][j]);
            }

            if j > 0 {
                val_max = val_max.max(dp1[i][j - 1]);
            }

            dp1[i][j] = val_max + matrix[i][j];
        }
    }

    let mut dp2 = vec![vec![i64::MIN; n]; n];
    dp2[n - 1][n - 1] = matrix[n - 1][n - 1];

    for i in (0..n).rev() {
        for j in (0..n).rev() {
            if i == n - 1 && j == n - 1 {
                continue;
            }

            let mut val_max = i64::MIN;

            if i < n - 1 {
                val_max = val_max.max(dp2[i + 1][j]);
            }

            if j < n - 1 {
                val_max = val_max.max(dp2[i][j + 1]);
            }

            dp2[i][j] = val_max + matrix[i][j];
        }
    }

    let mut ret = i64::MIN;
    let val = dp1[r][c] + dp2[r][c] - matrix[r][c];
    ret = ret.max(val);

    matrix[r][c] = i64::MIN;

    let mut dp3 = vec![vec![i64::MIN; n]; n];
    dp3[0][0] = matrix[0][0];

    for i in 0..n {
        for j in 0..n {

            if i == 0 && j == 0 {
                continue;
            }

            let mut val_max = i64::MIN;

            if i > 0 {
                val_max = val_max.max(dp3[i - 1][j]);
            }

            if j > 0 {
                val_max = val_max.max(dp3[i][j - 1]);
            }

            dp3[i][j] = val_max + matrix[i][j];
        }
    }

    writeln!(out, "{ret} {}", dp3[n - 1][n - 1]).unwrap();
}
