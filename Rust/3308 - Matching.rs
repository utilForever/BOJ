use io::Write;
use std::{cmp, io, str};

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

// Compare function
fn is_same(
    arr: &Vec<usize>,
    less_pos: &Vec<i64>,
    greater_pos: &Vec<i64>,
    left: usize,
    right: usize,
) -> bool {
    let diff = (right - left) as i64;

    if less_pos[left] != -1 && arr[(diff + less_pos[left]) as usize] >= arr[right] {
        return false;
    }

    if greater_pos[left] != -1 && arr[(diff + greater_pos[left]) as usize] <= arr[right] {
        return false;
    }

    true
}

// Reference: https://blog.cube219.me/posts/2021/solve-stock-price-prediction-problem-without-segment-tree/
// Reference: https://oi.edu.pl/static/attachment/20110713/ceoi-2011.pdf
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut s = vec![0; n];
    let mut h = vec![0; m];

    for i in 0..n {
        let val = scan.token::<usize>();
        s[val - 1] = i;
    }

    for i in 0..m {
        h[i] = scan.token::<usize>();
    }

    // Initialize values
    let mut prev_idx = vec![0_i64; n];
    let mut next_idx = vec![0_i64; n];
    let mut pos = vec![n as i64; n];
    let mut less_pos = vec![0_i64; n];
    let mut greater_pos = vec![0_i64; n];

    for i in 0..n {
        prev_idx[i] = i as i64 - 1;
        next_idx[i] = i as i64 + 1;
    }

    for i in 0..n {
        pos[s[i]] = cmp::min(pos[s[i]], i as i64);
    }

    // Calculate less_pos and greater_pos using doubly linked list like technique
    for i in (0..=n - 1).rev() {
        if next_idx[s[i]] == n as i64 {
            greater_pos[i] = -1;
        } else {
            greater_pos[i] = pos[next_idx[s[i]] as usize];
        }

        if prev_idx[s[i]] == -1 {
            less_pos[i] = -1;
        } else {
            less_pos[i] = pos[prev_idx[s[i]] as usize];
        }

        if prev_idx[s[i]] != -1 {
            next_idx[prev_idx[s[i]] as usize] = next_idx[s[i]];
        }

        if next_idx[s[i]] != n as i64 {
            prev_idx[next_idx[s[i]] as usize] = prev_idx[s[i]];
        }
    }

    // Calculate fail function
    let mut cmp = 0;
    let mut fail = vec![0; n];

    for i in 1..n {
        while cmp > 0 && !is_same(&s, &less_pos, &greater_pos, cmp, i) {
            cmp = fail[cmp - 1];
        }

        if is_same(&s, &less_pos, &greater_pos, cmp, i) {
            cmp += 1;
            fail[i] = cmp;
        }
    }

    // Use KMP alogrithm to find the answer
    let mut ret = Vec::new();
    cmp = 0;

    for i in 0..m {
        while cmp > 0 && !is_same(&h, &less_pos, &greater_pos, cmp, i) {
            cmp = fail[cmp - 1];
        }

        if is_same(&h, &less_pos, &greater_pos, cmp, i) {
            if cmp == n - 1 {
                ret.push(i - n + 1);
                cmp = fail[cmp];
            } else {
                cmp += 1;
            }
        }
    }

    writeln!(out, "{}", ret.len()).unwrap();
    for val in ret.iter() {
        write!(out, "{} ", val + 1).unwrap();
    }
    writeln!(out).unwrap();
}
