use io::Write;
use std::f64::consts::{PI, TAU};
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

fn gcd(mut first: i64, mut second: i64) -> i64 {
    if first < 0 {
        first = -first;
    }

    if second < 0 {
        second = -second;
    }

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
        std::mem::swap(&mut min, &mut max);
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

fn calculate(n: i64, m: i64) -> f64 {
    if n == 1 {
        return (PI / m as f64).cos();
    }

    if m == 1 {
        return 1.0;
    }

    let k = gcd(m, n);

    if k > 1 {
        return (PI / m as f64).cos() / (k as f64 * PI / (n as f64 * m as f64)).cos();
    }

    if m == 3 {
        let numerator = 1.5;
        let denominator = (PI / n as f64).cos()
            - 2.0 * (PI / n as f64 + TAU * (n / 3) as f64 / n as f64 + PI / 3.0).cos();

        return numerator / denominator;
    }

    if n == 3 {
        let angle_offset = if m % 3 == 1 { -PI / 3.0 } else { PI / 3.0 };
        let angle_common = PI / (3.0 * m as f64);
        let numerator = (PI / m as f64).cos() + (angle_common + angle_offset).cos();
        let denominator = angle_common.cos() + (angle_common + angle_offset).cos();

        return numerator / denominator;
    }

    if m == 4 {
        return 1.0
            / (2.0f64.sqrt() * (PI / (2.0 * n as f64)).cos() * (PI / (4.0 * n as f64)).cos());
    }

    if n == 4 {
        if m % 4 == 1 {
            return ((1.0 - (PI / (2.0 * m as f64)).sin())
                * ((PI / (2.0 * m as f64)).cos() + (PI / (2.0 * m as f64)).sin()))
                / (PI / (4.0 * m as f64)).cos();
        } else {
            return ((1.0 + (PI / (2.0 * m as f64)).sin())
                * ((PI / (2.0 * m as f64)).cos() - (PI / (2.0 * m as f64)).sin()))
                / (PI / (4.0 * m as f64)).cos();
        }
    }

    -100.0
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, _) = (scan.token::<usize>(), scan.token::<i64>());
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    let mut ret = 0.0;

    for i in 1..n {
        if nums[i] == 1 {
            continue;
        }

        ret -= calculate(nums[i - 1], nums[i]).ln();
    }

    writeln!(out, "{:.12}", ret).unwrap();
}
