use io::Write;
use std::{cmp, io, str};

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

fn calculate(
    max_coin: &mut Vec<Vec<usize>>,
    sum: &Vec<usize>,
    n: usize,
    i: usize,
    j: usize,
) -> usize {
    if j == 0 || i > n {
        return 0;
    }

    if max_coin[i][j] > 0 {
        return max_coin[i][j];
    }

    max_coin[i][j] = cmp::max(
        calculate(max_coin, sum, n, i, j - 1),
        sum[n] - sum[i - 1] - calculate(max_coin, sum, n, i + j, j * 2),
    );

    max_coin[i][j]
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut c = vec![0; n + 1];

    for i in 1..=n {
        c[i] = scan.token::<usize>();
    }

    let mut sum = vec![0; n + 1];
    for i in 1..=n {
        sum[i] += sum[i - 1] + c[i];
    }

    let mut max_coin = vec![vec![0; n + 1]; n + 1];
    writeln!(out, "{}", calculate(&mut max_coin, &sum, n, 1, 2)).unwrap();
}
