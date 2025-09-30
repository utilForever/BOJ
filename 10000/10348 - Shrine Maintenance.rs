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

const RADIUS: f64 = 1000.0;
const EPS: f64 = 1e-9;

fn check(gaps: &Vec<f64>, w: usize, bound: f64) -> bool {
    let n = gaps.len();

    if n == 0 {
        return true;
    }

    let mut len = vec![0; n];
    let mut sum = 0.0;
    let mut idx = 0;

    for i in 0..n {
        if idx < i {
            sum = 0.0;
            idx = i;
        }

        while idx < i + n && sum + gaps[idx % n] <= bound + EPS {
            sum += gaps[idx % n];
            idx += 1;
        }

        len[i] = idx - i;

        if idx > i {
            sum -= gaps[i % n];
        }
    }

    let mut num_gaps = vec![0; n];

    for i in 0..n {
        let start = (i + 1) % n;
        num_gaps[i] = 1 + len[start];
    }

    let mut w_log = 0;

    while (1usize << w_log) <= w {
        w_log += 1;
    }

    let mut dp = vec![vec![0; n]; w_log];
    dp[0].clone_from_slice(&num_gaps);

    for i in 1..w_log {
        for j in 0..n {
            let mid = (j + dp[i - 1][j]) % n;
            dp[i][j] = dp[i - 1][j] + dp[i - 1][mid];
        }
    }

    for i in 0..n {
        let mut covered = 0;
        let mut curr = i;
        let mut need = w;
        let mut idx = 0;

        while need > 0 {
            if need & 1 != 0 {
                covered += dp[idx][curr];
                curr = (curr + dp[idx][curr]) % n;
            }

            need >>= 1;
            idx += 1;
        }

        if covered >= n {
            return true;
        }
    }

    false
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let w = scan.token::<usize>();

        if w == 0 {
            break;
        }

        let (n, d) = (scan.token::<usize>(), scan.token::<usize>());
        let mut divisors = vec![0; d];

        for i in 0..d {
            divisors[i] = scan.token::<usize>();
        }

        let mut visited = vec![false; n + 1];

        for d in divisors {
            let mut idx = d;

            while idx <= n {
                visited[idx] = true;
                idx += d;
            }
        }

        let mut shrines = Vec::new();

        for i in 1..=n {
            if visited[i] {
                shrines.push(i);
            }
        }

        if w >= shrines.len() {
            writeln!(out, "{:.1}", 2.0 * RADIUS).unwrap();
            continue;
        }

        let mut gaps = vec![0.0; shrines.len()];

        for i in 0..shrines.len() {
            let shrine_curr = shrines[i];
            let shrine_next = shrines[(i + 1) % shrines.len()];
            let delta = if i + 1 < shrines.len() {
                shrine_next - shrine_curr
            } else {
                n + shrine_next - shrine_curr
            };

            let angle = std::f64::consts::PI * delta as f64 / n as f64;
            gaps[i] = 2.0 * RADIUS * angle.sin();
        }

        let mut left = 0.0;
        let mut right = gaps.iter().sum::<f64>();

        for _ in 0..60 {
            let mid = (left + right) / 2.0;

            if check(&gaps, w, mid) {
                right = mid;
            } else {
                left = mid;
            }
        }

        writeln!(out, "{:.1}", right + 2.0 * RADIUS).unwrap();
    }
}
