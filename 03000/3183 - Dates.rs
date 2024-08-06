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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let (d, m, y) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        if d == 0 && m == 0 && y == 0 {
            break;
        }

        // Check if the year is a leap year
        let is_leap = if y % 400 == 0 {
            true
        } else if y % 100 == 0 {
            false
        } else {
            y % 4 == 0
        };

        // Check if the day is valid
        let mut ret = match m {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => d >= 1 && d <= 31,
            4 | 6 | 9 | 11 => d >= 1 && d <= 30,
            2 => {
                if is_leap {
                    d >= 1 && d <= 29
                } else {
                    d >= 1 && d <= 28
                }
            }
            _ => false,
        };

        // Check if the month is valid
        if m < 1 || m > 12 {
            ret = false;
        }

        writeln!(out, "{}", if ret { "Valid" } else { "Invalid" }).unwrap();
    }
}
