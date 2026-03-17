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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut landscape = vec![vec![' '; m]; n];

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            landscape[i][j] = c;
        }
    }

    let mut height = vec![0; m];
    let mut left = vec![-1; m];
    let mut right = vec![m as i64; m];
    let mut stack = Vec::with_capacity(m);
    let mut ret = 0;

    for i in 0..n {
        for j in 0..m {
            if landscape[i][j] == '.' {
                height[j] += 1;
            } else {
                height[j] = 0;
            }
        }

        stack.clear();

        for j in 0..m {
            while let Some(&top) = stack.last() {
                if height[top] >= height[j] {
                    stack.pop();
                } else {
                    break;
                }
            }

            left[j] = if let Some(&top) = stack.last() {
                top as i64
            } else {
                -1
            };

            stack.push(j);
        }

        stack.clear();

        for j in (0..m).rev() {
            while let Some(&top) = stack.last() {
                if height[top] > height[j] {
                    stack.pop();
                } else {
                    break;
                }
            }

            right[j] = if let Some(&top) = stack.last() {
                top as i64
            } else {
                m as i64
            };

            stack.push(j);
        }

        for j in 0..m {
            if height[j] == 0 {
                continue;
            }

            let a = j as i64 - left[j];
            let b = right[j] - j as i64;

            ret += height[j] * (height[j] + 1) / 2 * a * b * (a + b) / 2;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
