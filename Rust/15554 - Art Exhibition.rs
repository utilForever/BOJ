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
    let mut pictures = vec![(0, 0); n + 1];

    for i in 1..=n {
        pictures[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    pictures.sort();

    let mut sum = vec![0; n + 1];

    for i in 1..=n {
        sum[i] = sum[i - 1] + pictures[i].1;
    }

    let mut ret = 0;
    let mut j = 0;

    for i in 1..=n {
        ret = ret.max(sum[i] - pictures[i].0 - (sum[j] - pictures[j + 1].0));

        if i < n && sum[i] - pictures[i + 1].0 < sum[j] - pictures[j + 1].0 {
            j = i;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
