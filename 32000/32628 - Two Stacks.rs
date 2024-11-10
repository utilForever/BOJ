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

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut backpack_first = vec![0; n + 1];
    let mut backpack_second = vec![0; n + 1];

    for i in 1..=n {
        backpack_first[i] = scan.token::<i64>();
    }

    for i in 1..=n {
        backpack_second[i] = scan.token::<i64>();
    }

    let sum_first = backpack_first.iter().sum::<i64>();
    let sum_second = backpack_second.iter().sum::<i64>();

    let mut prefix_sum_first = vec![0; n + 1];
    let mut prefix_sum_second = vec![0; n + 1];

    for i in (1..=n).rev() {
        prefix_sum_first[n - i + 1] = prefix_sum_first[n - i] + backpack_first[i];
        prefix_sum_second[n - i + 1] = prefix_sum_second[n - i] + backpack_second[i];
    }

    let mut ret = i64::MAX;

    for idx1 in 0..=k.min(n) {
        let idx2 = (k - idx1).min(n);

        let weight_first = sum_first - prefix_sum_first[idx1];
        let weight_second = sum_second - prefix_sum_second[idx2];
        let weight_max = weight_first.max(weight_second);

        ret = ret.min(weight_max);
    }

    writeln!(out, "{ret}").unwrap();
}
