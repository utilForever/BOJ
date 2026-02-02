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

const MAX: usize = 100_000;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let (n, m) = (scan.token::<usize>(), scan.token::<usize>());

        if n == 0 && m == 0 {
            break;
        }

        let mut cnt_electric_parts = vec![0; MAX + 1];
        let mut cnt_engineers = vec![0; MAX + 1];

        let mut sum_electric_parts = 0;
        let mut sum_demand = 0;

        for _ in 0..n {
            let (j, c) = (scan.token::<i64>(), scan.token::<usize>());

            cnt_electric_parts[c] += j;
            sum_electric_parts += j;
            sum_demand += j * c as i64;
        }

        let mut sum_engineers = 0;
        let mut sum_capacity = 0;

        for _ in 0..m {
            let (k, d) = (scan.token::<i64>(), scan.token::<usize>());

            cnt_engineers[d] += k;
            sum_engineers += k;
            sum_capacity += k * d as i64;
        }

        let limit = (sum_electric_parts as usize).min(MAX);
        let mut lhs = vec![0; limit + 1];
        let mut idx = 0;

        for c in (0..=MAX).rev() {
            if idx == limit {
                break;
            }

            let cnt = cnt_electric_parts[c];

            if cnt <= 0 {
                continue;
            }

            let take = ((limit - idx) as i64).min(cnt) as usize;

            for _ in 0..take {
                idx += 1;
                lhs[idx] = lhs[idx - 1] + c as i64;
            }
        }

        let mut prefix_cnt = vec![0; MAX + 1];
        let mut prefix_sum = vec![0; MAX + 1];

        for i in 0..=MAX {
            prefix_cnt[i] = if i == 0 {
                cnt_engineers[i]
            } else {
                prefix_cnt[i - 1] + cnt_engineers[i]
            };
            prefix_sum[i] = if i == 0 {
                cnt_engineers[i] * i as i64
            } else {
                prefix_sum[i - 1] + cnt_engineers[i] * i as i64
            };
        }

        let mut check = true;

        for i in 1..=limit {
            let cnt = sum_engineers - prefix_cnt[i - 1];
            let rhs = prefix_sum[i - 1] + (i as i64) * cnt;

            if lhs[i] > rhs {
                check = false;
                break;
            }
        }

        if sum_electric_parts as usize > MAX && sum_demand > sum_capacity {
            check = false;
        }

        writeln!(out, "{}", if check { "1" } else { "0" }).unwrap();
    }
}
