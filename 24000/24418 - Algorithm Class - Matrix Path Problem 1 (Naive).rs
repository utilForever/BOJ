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

fn matrix_path1_rec(matrix: &Vec<Vec<i64>>, i: usize, j: usize, cnt: &mut i64) -> i64 {
    if i == 0 || j == 0 {
        *cnt += 1;
        0
    } else {
        matrix[i][j]
            + matrix_path1_rec(matrix, i - 1, j, cnt).max(matrix_path1_rec(matrix, i, j - 1, cnt))
    }
}

fn matrix_path1(matrix: &Vec<Vec<i64>>, n: usize, cnt: &mut i64) -> i64 {
    matrix_path1_rec(matrix, n, n, cnt)
}

fn matrix_path2(matrix: &Vec<Vec<i64>>, n: usize, cnt: &mut i64) -> i64 {
    let mut dp = vec![vec![0; n + 1]; n + 1];

    for i in 0..=n {
        dp[i][0] = 0;
    }

    for j in 0..=n {
        dp[0][j] = 0;
    }

    for i in 1..=n {
        for j in 1..=n {
            *cnt += 1;
            dp[i][j] = matrix[i - 1][j - 1] + dp[i - 1][j].max(dp[i][j - 1]);
        }
    }

    dp[n][n]
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut matrix = vec![vec![0; n + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=n {
            matrix[i][j] = scan.token::<i64>();
        }
    }

    let mut ret1 = 0;
    let mut ret2 = 0;

    let _ = matrix_path1(&matrix, n, &mut ret1);
    let _ = matrix_path2(&matrix, n, &mut ret2);

    writeln!(out, "{ret1} {ret2}").unwrap();
}
