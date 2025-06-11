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

fn sum_floor_half(left: i128, right: i128, c: i128) -> i128 {
    let cnt = right - left + 1;
    let sum_c = c * cnt;
    let sum_k = (left + right) * cnt / 2;

    let parity_c = c & 1;
    let cnt_even = right / 2 - (left - 1) / 2;
    let cnt_odd = cnt - cnt_even;
    let ones = if parity_c == 0 { cnt_odd } else { cnt_even };

    (sum_k + sum_c - ones) / 2
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, e) = (scan.token::<i128>(), scan.token::<i128>());
    let (mut add_left, mut add_right) = (2, i128::MAX);
    let (mut sub_left, mut sub_right) = (i128::MIN, i128::MAX);
    let (mut mul_left, mut mul_right) = (1, i128::MAX);
    let (mut has_add, mut has_sub, mut has_mul) = (false, false, false);

    for _ in 0..n {
        let (r, k) = (scan.token::<i128>(), scan.token::<i128>());

        match r {
            1 => {
                has_add = true;

                let left = (k - e).max(2);
                let right = k + e;

                add_left = add_left.max(left);
                add_right = add_right.min(right);
            }
            2 => {
                has_sub = true;

                let left = k - e;
                let right = k + e;

                sub_left = sub_left.max(left);
                sub_right = sub_right.min(right);
            }
            3 => {
                has_mul = true;

                let left = (k - e).max(1);
                let right = k + e;

                mul_left = mul_left.max(left);
                mul_right = mul_right.min(right);
            }
            _ => unreachable!(),
        }
    }

    if !has_add && !has_mul {
        writeln!(out, "-1").unwrap();
        return;
    }

    let ret = if has_mul {
        let limit = (mul_right as f64).sqrt() as i128;
        let mut total = 0;

        for a in 1..=limit {
            let mut b_min = (mul_left + a - 1) / a;
            let mut b_max = mul_right / a;

            if b_min > b_max {
                continue;
            }

            if has_add {
                b_min = b_min.max(add_left - a);
                b_max = b_max.min(add_right - a);
            }

            if has_sub {
                b_min = b_min.max(a - sub_right);
                b_max = b_max.min(a - sub_left);
            }

            b_min = b_min.max(1);

            if b_min <= b_max {
                total += b_max - b_min + 1;
            }
        }

        for b in 1..=limit {
            let mut a_min = (mul_left + b - 1) / b;
            let mut a_max = mul_right / b;

            if has_add {
                a_min = a_min.max(add_left - b);
                a_max = a_max.min(add_right - b);
            }

            if has_sub {
                a_min = a_min.max(sub_left + b);
                a_max = a_max.min(sub_right + b);
            }

            a_min = a_min.max(limit + 1);

            if a_min <= a_max {
                total += a_max - a_min + 1;
            }
        }

        total
    } else if !has_sub {
        let cnt = add_right - add_left + 1;
        let sum_add = (add_left + add_right) * cnt / 2;

        sum_add - cnt
    } else {
        let mut total = 0;

        let a = sub_right + 2;
        let b = 2 - sub_left;
        let mut curr = add_left;

        while curr <= add_right {
            let is_small_upper = curr <= a - 1;
            let is_small_lower = curr <= b - 1;

            let next_upper = if is_small_upper { a } else { i128::MAX };
            let next_lower = if is_small_lower { b } else { i128::MAX };
            let seg_right = (next_upper.min(next_lower) - 1).min(add_right);

            let mut left = curr;
            let right = seg_right;

            if is_small_upper && is_small_lower {
                left = left.max(2);

                if left <= right {
                    let cnt_s = (left + right) * (right - left + 1) / 2;
                    let cnt = right - left + 1;

                    total += cnt_s - cnt;
                }
            } else if is_small_upper && !is_small_lower {
                left = left.max(sub_left + 2);

                if left <= right {
                    let cnt_s = (left + right) * (right - left + 1) / 2;
                    let sum_ceil = sum_floor_half(left, right, sub_left + 1);

                    total += cnt_s - sum_ceil;
                }
            } else if !is_small_upper && is_small_lower {
                left = left.max(2 - sub_right).max(2);

                if left <= right {
                    total += sum_floor_half(left, right, sub_right);
                }
            } else {
                if left <= right {
                    let sum_right = sum_floor_half(left, right, sub_right);
                    let sum_left = sum_floor_half(left, right, sub_left + 1);
                    let cnt = right - left + 1;

                    total += sum_right - sum_left + cnt;
                }
            };

            curr = seg_right + 1;
        }

        total
    };

    writeln!(out, "{ret}").unwrap();
}
