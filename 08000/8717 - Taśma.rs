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
    let mut prefix_sum = vec![0; n + 1];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    for i in 1..=n {
        prefix_sum[i] = prefix_sum[i - 1] + nums[i - 1];
    }

    let mut ret = i64::MAX;

    for i in 1..n {
        let sum_left = prefix_sum[i];
        let sum_right = prefix_sum[n] - prefix_sum[i];

        ret = ret.min((sum_left - sum_right).abs());
    }

    writeln!(out, "{ret}").unwrap();
}
