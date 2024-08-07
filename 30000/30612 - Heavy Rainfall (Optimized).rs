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

    let mut heights_left = vec![0; n];
    let mut heights_right = vec![0; n];
    let mut height_max = 0;

    for i in 0..n {
        heights_left[i] = height_max;

        height_max = if heights[i] == 0 {
            0
        } else {
            heights[i].max(height_max)
        };
    }

    height_max = 0;

    for i in (0..n).rev() {
        heights_right[i] = height_max;

        height_max = if heights[i] == 0 {
            0
        } else {
            heights[i].max(height_max)
        };
    }

    let mut ret = 0;

    for i in 0..n {
        if heights[i] == 0 {
            continue;
        }

        ret += (heights_left[i].min(heights_right[i]) - heights[i]).max(0);
    }

    writeln!(out, "{ret}").unwrap();
}
