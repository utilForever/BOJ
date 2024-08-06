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

fn calculate_minimum_awkwardness(
    u: &Vec<Vec<usize>>,
    pre_q: &Vec<usize>,
    cur_q: &mut Vec<usize>,
    left: usize,
    right: usize,
    p_left: usize,
    p_right: usize,
) {
    if left > right {
        return;
    }

    let mid = (left + right) / 2;
    let mut best_ret = (1_000_000, -1);

    for i in p_left..=cmp::min(mid, p_right) {
        best_ret = cmp::min(best_ret, (pre_q[i - 1] + u[i][mid], i as i32));
    }

    cur_q[mid] = best_ret.0;
    let p_mid = best_ret.1 as usize;

    calculate_minimum_awkwardness(u, pre_q, cur_q, left, mid - 1, p_left, p_mid);
    calculate_minimum_awkwardness(u, pre_q, cur_q, mid + 1, right, p_mid, p_right);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut u = vec![vec![0; n + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=n {
            u[i][j] = scan.token::<usize>();
        }
    }

    for i in 1..=n {
        for j in i + 1..=n {
            u[i][j] += u[i][j - 1];
        }
    }

    for j in 1..=n {
        for i in (1..=j - 1).rev() {
            u[i][j] += u[i + 1][j];
        }
    }

    let mut pre_q = vec![1_000_000; n + 1];
    let mut cur_q = vec![0; n + 1];

    pre_q[0] = 0;

    for _ in 0..k {
        calculate_minimum_awkwardness(&u, &pre_q, &mut cur_q, 1, n, 1, n);
        std::mem::swap(&mut pre_q, &mut cur_q);
    }

    writeln!(out, "{}", pre_q[n]).unwrap();
}
