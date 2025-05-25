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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let is_leap_year =
        |year: i64| -> bool { (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 };

    let get_days_in_month = |year: i64, month: i64| -> i64 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            _ => unreachable!(),
        }
    };

    let get_days_since_1970 = |year: i64, month: i64| -> i64 {
        let mut days = 0;

        for y in 1970..year {
            days += if is_leap_year(y) { 366 } else { 365 };
        }

        for m in 1..month {
            days += get_days_in_month(year, m);
        }

        days
    };

    let y = scan.token::<i64>();
    let mut ret = 0;

    for m in 1..=12 {
        let offset = get_days_since_1970(y, m) % 7;
        let day_first = (3 + offset) % 7;
        let days = get_days_in_month(y, m);
        let rows = (day_first + days + 6) / 7;

        ret += rows;
    }

    writeln!(out, "{ret}").unwrap();
}
