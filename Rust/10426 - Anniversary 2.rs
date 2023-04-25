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

fn is_leap_year(year: i64) -> bool {
    (year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)
}

fn days_in_month(year: i64, month: i64) -> i64 {
    match month {
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (date, n) = (scan.token::<String>(), scan.token::<i64>());
    let (year, month, day) = (
        date[0..4].parse::<i64>().unwrap(),
        date[5..7].parse::<i64>().unwrap(),
        date[8..10].parse::<i64>().unwrap(),
    );

    let mut day_after = day + n - 1;
    let mut month_after = month;
    let mut year_after = year;

    while day_after > days_in_month(year_after, month_after) {
        day_after -= days_in_month(year_after, month_after);
        month_after += 1;

        if month_after > 12 {
            month_after = 1;
            year_after += 1;
        }
    }

    writeln!(out, "{year_after}-{:02}-{:02}", month_after, day_after).unwrap();
}
