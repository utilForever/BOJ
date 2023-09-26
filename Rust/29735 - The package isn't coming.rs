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

    let (start, end) = (scan.token::<String>(), scan.token::<String>());
    let (n, t) = (scan.token::<i64>(), scan.token::<i64>());

    let (start_hour, start_min) = (
        start[0..2].parse::<i64>().unwrap(),
        start[3..5].parse::<i64>().unwrap(),
    );
    let (end_hour, end_min) = (
        end[0..2].parse::<i64>().unwrap(),
        end[3..5].parse::<i64>().unwrap(),
    );

    let time_start = start_hour * 60 + start_min;
    let time_end = end_hour * 60 + end_min;
    let num_packages_per_day = (time_end - time_start - 1) / t;

    let ret_days = n / num_packages_per_day;
    let ret_time = time_start + (n % num_packages_per_day + 1) * t;

    writeln!(out, "{ret_days}").unwrap();
    writeln!(out, "{:02}:{:02}", ret_time / 60, ret_time % 60).unwrap();
}
