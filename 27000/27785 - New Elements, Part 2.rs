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

#[derive(PartialEq, Eq)]
struct Fraction {
    numerator: i64,
    denominator: i64,
}

impl Fraction {
    fn new(numerator: i64, denominator: i64) -> Self {
        let mut g = gcd(numerator, denominator);

        if g < 0 {
            g = -g;
        }

        Self {
            numerator: numerator / g,
            denominator: denominator / g,
        }
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let left = self.numerator * other.denominator;
        let right = self.denominator * other.numerator;

        left.cmp(&right)
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let left = self.numerator * other.denominator;
        let right = self.denominator * other.numerator;

        left.partial_cmp(&right)
    }
}

#[inline(always)]
fn stern_brocot(left: Fraction, right: Fraction) -> Result<Fraction, String> {
    let mut start = Fraction::new(0, 1);
    let mut end = Fraction::new(1, 0);

    loop {
        let mid = Fraction::new(
            start.numerator + end.numerator,
            start.denominator + end.denominator,
        );

        if mid > left && mid < right {
            return Ok(mid);
        }

        if mid >= right {
            let mut start_new = 1;
            let mut end_new = 1;

            while Fraction::new(
                end_new * start.numerator + end.numerator,
                end_new * start.denominator + end.denominator,
            ) >= right
            {
                end_new *= 2;
            }

            while start_new <= end_new {
                let mid_new = (start_new + end_new) / 2;

                if Fraction::new(
                    mid_new * start.numerator + end.numerator,
                    mid_new * start.denominator + end.denominator,
                ) >= right
                {
                    start_new = mid_new + 1;
                } else {
                    end_new = mid_new - 1;
                }
            }

            end = Fraction::new(
                end_new * start.numerator + end.numerator,
                end_new * start.denominator + end.denominator,
            );
        } else {
            let mut start_new = 1;
            let mut end_new = 1;

            while Fraction::new(
                start.numerator + end_new * end.numerator,
                start.denominator + end_new * end.denominator,
            ) <= left
            {
                end_new *= 2;
            }

            while start_new <= end_new {
                let mid_new = (start_new + end_new) / 2;

                if Fraction::new(
                    start.numerator + mid_new * end.numerator,
                    start.denominator + mid_new * end.denominator,
                ) <= left
                {
                    start_new = mid_new + 1;
                } else {
                    end_new = mid_new - 1;
                }
            }

            start = Fraction::new(
                start.numerator + end_new * end.numerator,
                start.denominator + end_new * end.denominator,
            );
        }
    }
}

fn process(molecules: &Vec<(i64, i64)>, n: usize) -> Result<Fraction, String> {
    let mut left = Fraction::new(0, 1);
    let mut right = Fraction::new(1, 0);

    for b in 1..n {
        let (c_b, j_b) = molecules[b];

        for a in 0..b {
            let (c_a, j_a) = molecules[a];

            if c_a < c_b && j_a > j_b {
                let val = Fraction::new(c_b - c_a, j_a - j_b);
                right = right.min(val);
            } else if c_a > c_b && j_a < j_b {
                let val = Fraction::new(c_a - c_b, j_b - j_a);
                left = left.max(val);
            } else if c_a >= c_b && j_a >= j_b {
                return Err("IMPOSSIBLE".to_string());
            }
        }
    }

    if left >= right {
        return Err("IMPOSSIBLE".to_string());
    }

    return stern_brocot(left, right);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for i in 1..=t {
        let n = scan.token::<usize>();
        let mut molecules = vec![(0, 0); n];

        for j in 0..n {
            molecules[j] = (scan.token::<i64>(), scan.token::<i64>());
        }

        writeln!(
            out,
            "Case #{i}: {}",
            match process(&molecules, n) {
                Ok(fraction) => format!("{} {}", fraction.denominator, fraction.numerator),
                Err(message) => message,
            }
        )
        .unwrap();
    }
}
