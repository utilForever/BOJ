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
    let mut heights = vec![0; n + 1];

    for i in 1..=n {
        heights[i] = scan.token::<usize>();
    }

    let mut stack: Vec<usize> = Vec::new();
    let mut left = vec![0; n + 1];
    let mut right = vec![0; n + 1];

    for i in 1..=n {
        while !stack.is_empty() && heights[*stack.last().unwrap()] < heights[i] {
            stack.pop();
        }

        if !stack.is_empty() {
            let val = *stack.last().unwrap();
            left[i] = (i - val).pow(2) + (heights[i] - heights[val]).pow(2) + left[val];
        }

        stack.push(i);
    }

    stack.clear();

    for i in (1..=n).rev() {
        while !stack.is_empty() && heights[*stack.last().unwrap()] < heights[i] {
            stack.pop();
        }

        if !stack.is_empty() {
            let val = *stack.last().unwrap();
            right[i] = (val - i).pow(2) + (heights[i] - heights[val]).pow(2) + right[val];
        }

        stack.push(i);
    }

    let q = scan.token::<i64>();

    for _ in 0..q {
        let p = scan.token::<usize>();
        writeln!(out, "{}", left[p] + right[p]).unwrap();
    }
}
