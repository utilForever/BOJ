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

    let (n, x) = (scan.token::<usize>(), scan.token::<i64>());
    let mut heights = vec![0; n + 2];

    for i in 1..=n {
        heights[i] = scan.token::<i64>();
    }

    heights[0] = heights[1] - x;
    heights[n + 1] = heights[n] - x;

    let mut ret = i64::MAX;

    for i in 1..=n {
        // Case 1: Above and above
        let ret1 = (x - (heights[i] - heights[i - 1])).max(0)
            + (x - (heights[i + 1] - heights[i].max(heights[i - 1] + x))).max(0);
        // Case 2: Above and below
        let ret2 = (x - (heights[i] - heights[i - 1])).max(0)
            + (x - (heights[i].max(heights[i - 1] + x) - heights[i + 1])).max(0);
        // Case 3: Below and above
        let ret3 =
            (x - (heights[i - 1] - heights[i])).max(0) + (x - (heights[i + 1] - heights[i])).max(0);
        // Case 4: Below and below
        let ret4 = (x - (heights[i] - heights[i + 1])).max(0)
            + (x - (heights[i - 1] - (heights[i].max(heights[i + 1] + x)))).max(0);

        ret = ret.min(ret1).min(ret2).min(ret3).min(ret4);
    }

    writeln!(out, "{ret}").unwrap();
}
