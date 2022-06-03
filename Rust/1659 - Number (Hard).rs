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

// Reference: https://cubelover.tistory.com/8
// Reference: https://blog.myungwoo.kr/117
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (num_s, num_t) = (scan.token::<usize>(), scan.token::<usize>());
    let mut s = vec![0; num_s];
    let mut t = vec![0; num_t];

    for i in 0..num_s {
        s[i] = scan.token::<i64>();
    }

    for i in 0..num_t {
        t[i] = scan.token::<i64>();
    }

    let mut prefix_sum_s = vec![0; num_s + num_t + 1];
    let mut prefix_sum_t = vec![0; num_s + num_t + 1];
    let mut combined = vec![0; num_s + num_t + 1];
    let mut cnt = vec![0; num_s + num_t + 1];

    let mut idx_combined = 1;
    let mut idx_s = 0;
    let mut idx_t = 0;

    cnt[0] = 500_000;

    while idx_s < num_s && idx_t < num_t {
        prefix_sum_s[idx_combined] = prefix_sum_s[idx_combined - 1];
        prefix_sum_t[idx_combined] = prefix_sum_t[idx_combined - 1];
        cnt[idx_combined] = cnt[idx_combined - 1];

        if s[idx_s] < t[idx_t] {
            prefix_sum_s[idx_combined] += s[idx_s];
            combined[idx_combined] = s[idx_s];
            cnt[idx_combined] += 1;

            idx_combined += 1;
            idx_s += 1;
        } else {
            prefix_sum_t[idx_combined] += t[idx_t];
            combined[idx_combined] = t[idx_t];
            cnt[idx_combined] -= 1;

            idx_combined += 1;
            idx_t += 1;
        }
    }

    while idx_s < num_s {
        prefix_sum_s[idx_combined] = prefix_sum_s[idx_combined - 1] + s[idx_s];
        prefix_sum_t[idx_combined] = prefix_sum_t[idx_combined - 1];
        combined[idx_combined] = s[idx_s];
        cnt[idx_combined] = cnt[idx_combined - 1] + 1;

        idx_combined += 1;
        idx_s += 1;
    }

    while idx_t < num_t {
        prefix_sum_s[idx_combined] = prefix_sum_s[idx_combined - 1];
        prefix_sum_t[idx_combined] = prefix_sum_t[idx_combined - 1] + t[idx_t];
        combined[idx_combined] = t[idx_t];
        cnt[idx_combined] = cnt[idx_combined - 1] - 1;

        idx_combined += 1;
        idx_t += 1;
    }

    let mut last = vec![0; 1_000_001];
    let mut sum = vec![0; idx_combined];
    let mut idx_s = 0;
    let mut idx_t = 0;

    for i in 1..idx_combined {
        sum[i] = i64::MAX;

        if cnt[i] == cnt[i - 1] + 1 {
            idx_s += 1;
        } else {
            idx_t += 1;
        }

        let val = last[cnt[i]];

        if val > 0 {
            sum[i] = sum[val]
                + ((prefix_sum_s[i] - prefix_sum_s[val]) - (prefix_sum_t[i] - prefix_sum_t[val]))
                    .abs();
        } else if cnt[i] == 500_000 {
            sum[i] = (prefix_sum_s[i] - prefix_sum_t[i]).abs();
        }

        let mut ret = i64::MAX;

        if cnt[i] == cnt[i - 1] + 1 {
            if idx_t != 0 {
                ret = cmp::min(ret, (combined[i] - t[idx_t - 1]).abs());
            }

            if idx_t != num_t {
                ret = cmp::min(ret, (combined[i] - t[idx_t]).abs());
            }
        } else {
            if idx_s != 0 {
                ret = cmp::min(ret, (combined[i] - s[idx_s - 1]).abs());
            }

            if idx_s != num_s {
                ret = cmp::min(ret, (combined[i] - s[idx_s]).abs());
            }
        }

        sum[i] = cmp::min(sum[i], sum[i - 1] + ret);
        last[cnt[i]] = i;
    }

    writeln!(out, "{}", sum[idx_combined - 1]).unwrap();
}
