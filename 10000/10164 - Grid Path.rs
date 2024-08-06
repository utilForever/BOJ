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

fn combination(factorial: &Vec<i128>, mut n: usize, mut r: usize) -> i128 {
    if n < r {
        std::mem::swap(&mut n, &mut r);
    }

    factorial[n] / (factorial[r] * factorial[n - r])
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );

    let mut factorial = vec![1; n + m + 1];
    factorial[0] = 1;

    for i in 1..=n + m {
        factorial[i] = factorial[i - 1] * i as i128;
    }

    if k == 0 {
        writeln!(out, "{}", combination(&factorial, n + m - 2, n - 1)).unwrap();
        return;
    }

    let pos_x = (k - 1) % m + 1;
    let pos_y = if k % m == 0 { k / m } else { k / m + 1 };
    let ret1 = combination(&factorial, pos_x + pos_y - 2, pos_x - 1);
    let ret2 = combination(&factorial, n + m - pos_x - pos_y, n - pos_y);

    writeln!(out, "{}", ret1 * ret2).unwrap();
}
