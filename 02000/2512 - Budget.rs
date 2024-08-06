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

    let n = scan.token::<usize>();
    let mut budgets = vec![0; n];

    for i in 0..n {
        budgets[i] = scan.token::<i64>();
    }

    let m = scan.token::<i64>();

    if budgets.iter().sum::<i64>() <= m {
        writeln!(out, "{}", budgets.iter().max().unwrap()).unwrap();
        return;
    }

    let mut left = 0;
    let mut right = 1_000_000_000;

    while left < right {
        let mid = (left + right) / 2;
        let mut sum = 0;

        for budget in budgets.iter() {
            sum += budget.min(&mid);
        }

        if sum > m {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    writeln!(out, "{}", left - 1).unwrap();
}
