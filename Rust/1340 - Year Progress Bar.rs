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

    let (month, day, year, time) = (
        scan.token::<String>(),
        scan.token::<String>(),
        scan.token::<i64>(),
        scan.token::<String>(),
    );
    let month = match month.as_str() {
        "January" => 1,
        "February" => 2,
        "March" => 3,
        "April" => 4,
        "May" => 5,
        "June" => 6,
        "July" => 7,
        "August" => 8,
        "September" => 9,
        "October" => 10,
        "November" => 11,
        "December" => 12,
        _ => unreachable!(),
    };
    let day = day[..day.len() - 1].parse::<i64>().unwrap();
    let time = time
        .split(':')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let time = time[0] * 60 + time[1];

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

    let mut total = 0;

    for i in 1..=12 {
        let days = get_days_in_month(year, i);
        total += 60 * 24 * days;
    }

    let mut partial = 0;

    for i in 1..month {
        let days = get_days_in_month(year, i);
        partial += 60 * 24 * days;
    }

    partial += 60 * 24 * (day - 1);
    partial += time;

    writeln!(out, "{:.9}", (partial as f64 * 100.0) / total as f64).unwrap();
}
