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

    let (a, b) = (scan.token::<String>(), scan.token::<String>());
    let (c, d) = (scan.token::<String>(), scan.token::<String>());

    let (first_ad, first_year) = if a == "AD" {
        (true, b.parse::<i64>().unwrap())
    } else {
        (false, a.parse::<i64>().unwrap())
    };
    let (second_ad, second_year) = if c == "AD" {
        (true, d.parse::<i64>().unwrap())
    } else {
        (false, c.parse::<i64>().unwrap())
    };

    let ret = match (first_ad, second_ad) {
        (true, true) => first_year.max(second_year) - first_year.min(second_year),
        (true, false) => first_year + second_year - 1,
        (false, true) => first_year + second_year - 1,
        (false, false) => first_year.max(second_year) - first_year.min(second_year),
    };

    writeln!(out, "{ret}").unwrap();
}
