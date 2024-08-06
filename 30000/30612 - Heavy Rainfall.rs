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
    let mut heights = vec![0; n];

    for i in 0..n {
        heights[i] = scan.token::<i64>();
    }

    let mut ret = 0;

    for i in 1..n - 1 {
        let mut bound_left = i - 1;
        let mut bound_right = i + 1;

        if heights[i] == 0 || heights[bound_left] == 0 || heights[bound_right] == 0 {
            continue;
        }

        for left in (0..=i - 1).rev() {
            if heights[left] == 0 {
                break;
            }

            if heights[left] > heights[bound_left] {
                bound_left = left;
            }
        }

        for right in i + 1..n {
            if heights[right] == 0 {
                break;
            }

            if heights[right] > heights[bound_right] {
                bound_right = right;
            }
        }

        if heights[bound_left] < heights[i] || heights[i] > heights[bound_right] {
            continue;
        }

        ret += heights[bound_left].min(heights[bound_right]) - heights[i];
    }

    writeln!(out, "{ret}").unwrap();
}
