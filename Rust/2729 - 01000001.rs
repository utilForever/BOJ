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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (a, b) = (scan.token::<String>(), scan.token::<String>());

        // Convert binary to decimal
        let mut a_converted = 0;
        let mut b_converted = 0;

        for (i, c) in a.chars().rev().enumerate() {
            if c == '1' {
                a_converted += 2i128.pow(i as u32);
            }
        }

        for (i, c) in b.chars().rev().enumerate() {
            if c == '1' {
                b_converted += 2i128.pow(i as u32);
            }
        }

        let mut sum = a_converted + b_converted;

        if sum == 0 {
            writeln!(out, "0").unwrap();
            continue;
        }

        // Convert decimal to binary
        let mut ret = String::new();

        while sum > 0 {
            if sum % 2 == 0 {
                ret.push('0');
            } else {
                ret.push('1');
            }

            sum /= 2;
        }

        ret = ret.chars().rev().collect();

        writeln!(out, "{ret}").unwrap();
    }
}
