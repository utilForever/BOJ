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

    let n = scan.token::<i64>();

    for _ in 0..n {
        let line = scan.line().trim().to_string();
        let tokens = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        if tokens.len() != 3 {
            writeln!(out, "18").unwrap();
            break;
        }

        let (x, k, h) = (tokens[0], tokens[1], tokens[2]);
        let rate_normal = x * (k - h).min(140);
        let rate_overtime = x * (k - h - 140).max(0) * 3 / 2;
        let rate_holiday = x * h * 2;
        let rate_total = (rate_normal + rate_overtime + rate_holiday).to_string();

        let mut ret = String::new();

        for (i, c) in rate_total.chars().rev().enumerate() {
            if i != 0 && i % 3 == 0 {
                ret.push(',');
            }

            ret.push(c);
        }

        let ret = ret.chars().rev().collect::<String>();

        writeln!(out, "{ret}").unwrap();
    }
}
