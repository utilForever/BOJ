use io::Write;
use std::{cmp, io, mem, str};

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

// Reference: https://www.csc.kth.se/~austrin/icpc/finals2013solutions.pdf
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (mut a, mut b, m, n) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut pond = vec![vec![0; n]; m];

    if a > b {
        mem::swap(&mut a, &mut b);
    }

    for i in 0..m {
        for j in 0..n {
            pond[i][j] = scan.token::<i64>();
        }
    }

    let area_chest = (m * n) as i64;
    let mut ans = 0;

    for i in 0..m {
        let mut depth = vec![1_000_000_007_i64; n];
        let mut stack = Vec::new();
        let mut left = vec![0; n];
        let mut right = vec![0; n];

        for j in 0..n {
            depth[j] = pond[i][j];
        }

        for j in i..m {
            if j >= i + b {
                break;
            }

            let height = j - i + 1;

            for k in 0..n {
                depth[k] = cmp::min(depth[k], pond[j][k]);
            }

            for k in 0..n {
                if stack.is_empty() || depth[*stack.last().unwrap() as usize] < depth[k] {
                    left[k] = k;
                    stack.push(k);
                }

                while !stack.is_empty() && depth[*stack.last().unwrap() as usize] >= depth[k] {
                    let index = stack.pop().unwrap() as usize;
                    right[index] = k;
                }

                if stack.is_empty() {
                    left[k] = 0;
                    stack.push(k);
                } else {
                    left[k] = *stack.last().unwrap() as usize + 1;
                    stack.push(k);
                }
            }

            while !stack.is_empty() {
                let index = stack.pop().unwrap() as usize;
                right[index] = n;
            }

            for k in 0..n {
                let width = cmp::min(if height > a { a } else { b }, right[k] - left[k]);
                let area = (width * height) as i64;
                let volume = depth[k] * area_chest;

                ans = match volume % (area_chest - area) == 0 {
                    true => cmp::max(ans, area * (volume / (area_chest - area) - 1)),
                    false => cmp::max(ans, area * (volume / (area_chest - area))),
                }
            }
        }
    }

    writeln!(out, "{ans}").unwrap();
}
