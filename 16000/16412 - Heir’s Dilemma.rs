use io::Write;
use std::{collections::HashSet, io, str};

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

    let (l, h) = (scan.token::<i64>(), scan.token::<i64>());
    let mut ret = 0;

    for num in l..=h {
        let mut divisors = HashSet::new();
        let mut val = num;
        let mut check = true;

        while val > 0 {
            let digit = val % 10;

            if digit == 0 || divisors.contains(&digit) || num % digit != 0 {
                check = false;
                break;
            }

            divisors.insert(digit);
            val /= 10;
        }

        if check {
            ret += 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
