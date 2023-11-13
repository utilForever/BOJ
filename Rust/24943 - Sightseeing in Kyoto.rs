use io::Write;
use std::{io, str, vec};

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

fn calculate_slope(values: &Vec<i64>, x1: usize, x2: usize) -> f64 {
    (values[x1] - values[x2]) as f64 / (x1 as i64 - x2 as i64) as f64
}

fn build_convex(values: &Vec<i64>, size: usize) -> (Vec<usize>, usize) {
    let mut stack = vec![0; size + 1];
    let mut idx = 0;

    for i in 1..=size {
        while idx > 1 {
            let slope1 = calculate_slope(values, stack[idx - 1], stack[idx]);
            let slope2 = calculate_slope(values, stack[idx], i);

            if slope1 < slope2 {
                break;
            }

            idx -= 1;
        }

        idx += 1;
        stack[idx] = i;
    }

    (stack, idx)
}

// Reference: https://jh05013.github.io/%EB%AC%B8%EC%A0%9C%ED%92%80%EC%9D%B4/BOJ%2018796%20%EC%9D%B4%EB%8F%99%ED%95%98%EA%B8%B0%204/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (h, w) = (scan.token::<usize>(), scan.token::<usize>());
    let mut a = vec![0; h + 1];
    let mut b = vec![0; w + 1];

    for i in 1..=h {
        a[i] = scan.token::<i64>();
    }

    for i in 1..=w {
        b[i] = scan.token::<i64>();
    }

    let (stack_a, idx_a) = build_convex(&a, h);
    let (stack_b, idx_b) = build_convex(&b, w);

    let mut pos_x = 1;
    let mut pos_y = 1;
    let mut ret = 0;

    while pos_x < idx_a && pos_y < idx_b {
        let slope1 = calculate_slope(&a, stack_a[pos_x + 1], stack_a[pos_x]);
        let slope2 = calculate_slope(&b, stack_b[pos_y + 1], stack_b[pos_y]);

        if slope1 < slope2 {
            ret += (stack_a[pos_x + 1] - stack_a[pos_x]) as i64 * b[stack_b[pos_y]];
            pos_x += 1;
        } else {
            ret += (stack_b[pos_y + 1] - stack_b[pos_y]) as i64 * a[stack_a[pos_x]];
            pos_y += 1;
        }
    }

    while pos_x < idx_a {
        ret += (stack_a[pos_x + 1] - stack_a[pos_x]) as i64 * b[stack_b[pos_y]];
        pos_x += 1;
    }

    while pos_y < idx_b {
        ret += (stack_b[pos_y + 1] - stack_b[pos_y]) as i64 * a[stack_a[pos_x]];
        pos_y += 1;
    }

    writeln!(out, "{ret}").unwrap();
}
