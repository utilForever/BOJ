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

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;

    if min == 0 && max == 0 {
        return 0;
    } else if min == 0 {
        return max;
    } else if max == 0 {
        return min;
    }

    if min > max {
        let val = max;

        max = min;
        min = val;
    }

    loop {
        let res = max % min;

        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<u32>(),
    );
    let mut count_nums = vec![0; n + 1];

    for _ in 0..m - 1 {
        let (a, b, c) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );
        count_nums[a] += 1;
        count_nums[b] += 1;
        count_nums[c] += 1;
    }

    let mut cases_max = 0;
    let cases_total = (3 * m).pow(k + 1);
    let mut ret = [0; 3];

    for num1 in 1..=n - 2 {
        for num2 in num1 + 1..=n - 1 {
            for num3 in num2 + 1..=n {
                let cnt = count_nums[num1] + count_nums[num2] + count_nums[num3] + 3;
                let cases_local = (3 * m - cnt).pow(k) * cnt;

                if cases_local > cases_max {
                    cases_max = cases_local;
                    ret = [num1, num2, num3];
                }
            }
        }
    }

    let gcd = gcd(cases_max, cases_total);

    writeln!(out, "{} {}", cases_max / gcd, cases_total / gcd).unwrap();

    for val in ret {
        write!(out, "{val} ").unwrap();
    }

    writeln!(out).unwrap();
}
