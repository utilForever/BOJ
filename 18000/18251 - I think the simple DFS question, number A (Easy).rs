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

fn process_inorder(
    weights: &Vec<i64>,
    order: &mut Vec<(i64, usize)>,
    depth: usize,
    depth_max: &mut usize,
    idx: usize,
    n: usize,
) {
    if idx > n {
        return;
    }

    process_inorder(weights, order, depth + 1, depth_max, 2 * idx, n);

    order.push((weights[idx - 1], depth));
    *depth_max = (*depth_max).max(depth);

    process_inorder(weights, order, depth + 1, depth_max, 2 * idx + 1, n);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut weights = vec![0; n];

    for i in 0..n {
        weights[i] = scan.token::<i64>();
    }

    let mut order = Vec::with_capacity(n);
    let mut depth_max = 0;

    process_inorder(&weights, &mut order, 0, &mut depth_max, 1, n);

    let mut ret = i64::MIN;

    for i in 0..=depth_max {
        for j in i..=depth_max {
            let mut val = 0;

            for k in 0..n {
                if order[k].1 < i || order[k].1 > j {
                    continue;
                }

                val += order[k].0;
                ret = ret.max(val);

                if val < 0 {
                    val = 0;
                }
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
