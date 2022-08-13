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
        scan.token::<i32>(),
        scan.token::<i32>(),
        scan.token::<i32>(),
    );
    let (year2, month2, day2) = (
        scan.token::<i32>(),
        scan.token::<i32>(),
        scan.token::<i32>(),
    );

    let mut age1 = 0;
    let age2 = year2 - year1 + 1;
    let age3 = age2 - 1;

    if year1 < year2 {
        if month1 < month2 || (month1 == month2 && day1 <= day2) {
            age1 = year2 - year1;
        } else {
            age1 = year2 - year1 - 1;
        }
    }

    writeln!(out, "{}\n{}\n{}", age1, age2, age3).unwrap();
}
