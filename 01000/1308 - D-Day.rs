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

    let (year1, month1, day1) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let (year2, month2, day2) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );

    // Print "gg" if year2 >= year1 + 1000 && month2 >= month1 && day2 >= day1
    if (year2 > year1 + 1000)
        || (year2 == year1 + 1000 && month2 > month1)
        || (year2 == year1 + 1000 && month2 == month1 && day2 >= day1)
    {
        writeln!(out, "gg").unwrap();
        return;
    }

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

    // Calculate the number of days between the two dates
    let mut ret = 0;

    // If year1 == year2, we can just calculate the difference in days
    if year1 == year2 {
        if month1 == month2 {
            ret += day2 - day1;
        } else {
            ret += get_days_in_month(year1, month1) - day1 + 1;

            for month in month1 + 1..month2 {
                ret += get_days_in_month(year1, month);
            }

            ret += day2 - 1;
        }

        writeln!(out, "D-{ret}").unwrap();
        return;
    }

    // First, add rest of the days in year1
    ret += get_days_in_month(year1, month1) - day1 + 1;

    for month in month1 + 1..=12 {
        ret += get_days_in_month(year1, month);
    }

    // Second, add all days in the years between year1 and year2
    for year in year1 + 1..year2 {
        ret += if is_leap_year(year) { 366 } else { 365 };
    }

    // Last, add days from 1st of January to day2 in year2
    for month in 1..month2 {
        ret += get_days_in_month(year2, month);
    }

    ret += day2 - 1;

    writeln!(out, "D-{ret}").unwrap();
}
