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

    let m = scan.token::<usize>();
    let mut heights = vec![0; m];

    for i in 0..m {
        heights[i] = scan.token::<i64>();
    }

    if m == 1 {
        writeln!(out, "1").unwrap();
        return;
    }

    heights.dedup();

    if heights.len() == 1 {
        writeln!(out, "1").unwrap();
        return;
    }

    let mut idx = 0;
    let mut ret = 0;

    while idx < heights.len() - 1 {
        let mut cnt = 1;
        let diff = heights[idx + 1] - heights[idx];

        if diff > 0 {
            while idx < heights.len() - 1 && heights[idx + 1] - heights[idx] > 0 {
                cnt += 1;
                idx += 1;
            }
        } else {
            while idx < heights.len() - 1 && heights[idx + 1] - heights[idx] < 0 {
                cnt += 1;
                idx += 1;
            }
        }

        ret = ret.max(cnt);
    }

    writeln!(out, "{ret}").unwrap();
}
