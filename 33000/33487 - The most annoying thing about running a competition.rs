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

fn check(
    dp: &mut Vec<f64>,
    lengths: &Vec<usize>,
    prefix_sum: &Vec<usize>,
    t: f64,
    n: usize,
    l: usize,
) -> bool {
    let mut deque = vec![0; 1_000_001];
    let mut front = 0;
    let mut back = 1;
    let mut j_max = 1;
    let mut k_min = 0;

    for i in 1..n {
        dp[i] = dp[i - 1].max(if lengths[i] == 1 {
            l as f64 - 1.0
        } else {
            (l - lengths[i]) as f64 / (lengths[i] - 1) as f64
        });

        if dp[i] <= t {
            deque[back] = i as i64;
            back += 1;
            continue;
        }

        while j_max < i {
            let val = prefix_sum[i] - prefix_sum[j_max - 1] + i - j_max;

            if val > l {
                j_max += 1;
            } else {
                break;
            }
        }

        while k_min < i - 1 {
            let val = prefix_sum[i] - prefix_sum[k_min] + i - k_min - 1;

            if l as f64 - val as f64 <= (t - 1.0) * (i as f64 - k_min as f64 - 1.0) {
                k_min += 1;
            } else {
                break;
            }
        }

        while front != back && deque[front] < j_max as i64 - 1 {
            front += 1;
        }

        if front != back && deque[front] <= k_min as i64 - 1 {
            dp[i] = 0.0;
            deque[back] = i as i64;
            back += 1;
        } else {
            dp[i] = l as f64 + 1.0;
        }
    }

    let mut val = lengths[n];
    let mut ratio = dp[n - 1];

    for i in (1..n).rev() {
        val += lengths[i] + 1;

        if val > l {
            break;
        }

        ratio = ratio.min(dp[i - 1].max(1.0));
    }

    ratio <= t
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, l) = (scan.token::<usize>(), scan.token::<usize>());
    let mut lengths = vec![0; n + 1];
    let mut prefix_sum = vec![0; n + 1];

    for i in 1..=n {
        lengths[i] = scan.token::<usize>();
        prefix_sum[i] = prefix_sum[i - 1] + lengths[i];
    }

    let mut dp = vec![0.0; n + 1];
    let mut val_min = 0.0;
    let mut val_max = l as f64;

    while val_max - val_min > val_min.max(1.0) * 1e-10 {
        let mid = (val_min + val_max) / 2.0;

        if check(&mut dp, &lengths, &prefix_sum, mid, n, l) {
            val_max = mid;
        } else {
            val_min = mid;
        }
    }

    writeln!(out, "{:.12}", val_max).unwrap();
}
