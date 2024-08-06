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

fn convert(s: &[char], left: usize, right: usize) -> i64 {
    let mut ret = 0;

    for i in left..=right {
        ret = ret * 10 + (s[i] as i64 - '0' as i64);
    }

    ret
}

fn calculate(s: &[char], first: i64, second: i64, last: i64, left: usize, right: usize) -> i64 {
    // Check numbers beacuse (first, second, last) are arithmetic progression
    if first >= second || second >= last {
        return i64::MAX;
    }

    // Only 3 numbers
    if left > right {
        // last = second * f_a (f_a >= 2)
        if last % second == 0 && last / second >= 2 {
            return last / second;
        }

        return i64::MAX;
    }

    // d is common difference of arithmetic progression
    let d = second - first;
    let mut curr = 0;
    let mut prev = second;

    // Check numbers between second and last
    for i in left..=right {
        if curr == 0 && s[i] == '0' {
            return i64::MAX;
        }

        curr = curr * 10 + (s[i] as i64 - '0' as i64);

        if curr == prev + d {
            prev = curr;
            curr = 0;
            continue;
        }

        if curr >= 100_000_000 && curr <= 999_999_999 {
            return i64::MAX;
        }
    }

    // last = prev * f_a (f_a >= 2)
    if curr == 0 && last % prev == 0 && last / prev >= 2 {
        return last / prev;
    }

    i64::MAX
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let s = scan.token::<String>();
    let s = s.chars().collect::<Vec<_>>();
    let n = s.len();
    let mut ret = i64::MAX;

    // i : first number's position according to the digit
    // j : second number's position according to the digit
    // k : last number's position according to the digit
    for i in 0..9 {
        for j in i + 1..i + 10 {
            for k in 1..=9 {
                // Check boundary
                if j + k + 1 > n {
                    continue;
                }

                // Can't start with 0
                if s[i + 1] == '0' {
                    continue;
                }

                if s[n - k] == '0' {
                    continue;
                }

                // Convert string to number
                let first = convert(&s, 0, i);
                let second = convert(&s, i + 1, j);
                let last = convert(&s, n - k, n - 1);

                ret = ret.min(calculate(&s, first, second, last, j + 1, n - k - 1));
            }
        }
    }

    writeln!(out, "{}", if ret == i64::MAX { 0 } else { ret }).unwrap();
}
