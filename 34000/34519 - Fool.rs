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
    let mut times = vec![0; n];

    for i in 0..n {
        times[i] = scan.token::<i64>();
    }

    let mut sum_left = vec![0; n + 1];

    for i in 1..=n {
        sum_left[i] = sum_left[i - 1] + times[i - 1] / 2;
    }

    let mut val_left = i64::MAX / 4;
    let mut min_left = vec![0; n + 1];

    for i in 1..=n {
        let val = times[i - 1] - sum_left[i];

        val_left = val_left.min(val);
        min_left[i] = sum_left[i] + val_left;
    }

    let mut val_right = i64::MAX / 4;
    let mut temp = vec![0; n + 1];
    let mut min_right = vec![0; n + 1];

    for i in (1..=n).rev() {
        let val = times[i - 1] + sum_left[i - 1];

        val_right = val_right.min(val);
        temp[i] = val_right;
    }

    for i in 1..=n {
        min_right[i] = temp[i] - sum_left[i - 1];
    }

    let mut ret = 0;

    for i in 1..=n {
        let left = min_left[i];
        let right = min_right[i];
        let mid = if i > 1 && i < n {
            min_left[i - 1].max(min_right[i + 1]) + (times[i - 1] / 4)
        } else {
            i64::MAX
        };

        ret = ret.max(left.min(right).min(mid));
    }

    writeln!(out, "{ret}").unwrap();
}
