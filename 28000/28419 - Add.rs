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
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    let sum_odd = nums
        .iter()
        .enumerate()
        .filter(|(i, _)| *i % 2 == 0)
        .map(|(_, x)| *x)
        .sum::<i64>();
    let sum_even = nums
        .iter()
        .enumerate()
        .filter(|(i, _)| *i % 2 == 1)
        .map(|(_, x)| *x)
        .sum::<i64>();

    if n == 3 {
        writeln!(
            out,
            "{}",
            if sum_odd <= sum_even {
                (sum_odd - sum_even).abs()
            } else {
                -1
            }
        )
        .unwrap();
        return;
    }

    writeln!(out, "{}", (sum_odd - sum_even).abs()).unwrap();
}
