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

fn hungarian_algorithm(cost: &Vec<Vec<f64>>) -> (f64, Vec<usize>) {
    let n = cost.len();
    let mut u = vec![0.0; n + 1];
    let mut v = vec![0.0; n + 1];
    let mut p = vec![0; n + 1];
    let mut way = vec![0; n + 1];

    for i in 1..=n {
        p[0] = i;

        let mut j0 = 0;
        let mut minv = vec![f64::INFINITY; n + 1];
        let mut used = vec![false; n + 1];

        loop {
            used[j0] = true;

            let i0 = p[j0];
            let mut delta = f64::INFINITY;
            let mut j1 = 0;

            for j in 1..=n {
                if used[j] {
                    continue;
                }

                let cur = cost[i0 - 1][j - 1] - u[i0] - v[j];

                if cur < minv[j] {
                    minv[j] = cur;
                    way[j] = j0;
                }

                if minv[j] < delta {
                    delta = minv[j];
                    j1 = j;
                }
            }

            for j in 0..=n {
                if used[j] {
                    u[p[j]] += delta;
                    v[j] -= delta;
                } else {
                    minv[j] -= delta;
                }
            }

            j0 = j1;

            if p[j0] == 0 {
                loop {
                    let j1 = way[j0];

                    p[j0] = p[j1];
                    j0 = j1;

                    if j0 == 0 {
                        break;
                    }
                }

                break;
            }
        }
    }

    let mut match_col = vec![0_usize; n + 1];

    for j in 1..=n {
        if p[j] > 0 {
            match_col[p[j]] = j;
        }
    }

    let total_cost = (1..=n).map(|i| cost[i - 1][match_col[i] - 1]).sum::<f64>();

    (total_cost, match_col[1..].iter().map(|&c| c - 1).collect())
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut baseline = 0;

    for _ in 0..n {
        let (x, y) = (scan.token::<i64>(), scan.token::<i64>());

        if x < 0 {
            left.push((x, y));
            baseline += x.abs();
        } else if x > 0 {
            right.push((x, y));
            baseline += x;
        }
    }

    let len = left.len().max(right.len());

    if len == 0 {
        writeln!(out, "0.000").unwrap();
        return;
    }

    let mut gain = vec![vec![0.0; len]; len];

    for i in 0..left.len() {
        for j in 0..right.len() {
            let (x1, y1) = left[i];
            let (x2, y2) = right[j];
            let dist = (((x1 + x2).pow(2) + (y1 - y2).pow(2)) as f64).sqrt();
            let g = x1.abs() as f64 + x2.abs() as f64 - dist;

            gain[i][j] = g.max(0.0);
        }
    }

    let mut cost = vec![vec![0.0; len]; len];

    for i in 0..len {
        for j in 0..len {
            cost[i][j] = -gain[i][j];
        }
    }

    let (cost_min, _) = hungarian_algorithm(&cost);
    let gain_max = -cost_min;

    let ret = baseline as f64 - gain_max;
    writeln!(out, "{:.3}", ret + 1e-9).unwrap();
}
