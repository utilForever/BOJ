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
    let mut heights = vec![0; n];

    for i in 0..n {
        heights[i] = scan.token::<i64>();
    }

    let mut left = vec![0; n];
    let mut prefix = i64::MAX;

    for i in 0..n {
        prefix = prefix.min(heights[i] - i as i64);
        left[i] = prefix + i as i64;
    }

    let mut right = vec![0; n];
    let mut suffix = i64::MAX;

    for i in (0..n).rev() {
        suffix = suffix.min(heights[i] + i as i64);
        right[i] = suffix - i as i64;
    }

    let mut ret = 0;

    for i in 0..n {
        let height = left[i].min(right[i]);
        let bound = (i + 1).min(n - i) as i64;

        ret += height.min(bound);
    }

    writeln!(out, "{ret}").unwrap();
}
