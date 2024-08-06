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

    let n = scan.token::<i128>();

    if n < 100 {
        writeln!(out, "{n}").unwrap();
        return;
    }

    let mut cnt = 99;

    for i in 100..=n.min(999) {
        let first = i / 100;
        let second = (i / 10) % 10;
        let third = i % 10;

        if first + third == 2 * second {
            cnt += 1;
        }
    }

    for i in 1000..=n.min(9999) {
        let first = i / 1000;
        let second = (i / 100) % 10;
        let third = (i / 10) % 10;
        let fourth = i % 10;

        if first + third == 2 * second && second + fourth == 2 * third {
            cnt += 1;
        }
    }

    for i in 10000..=n.min(99999) {
        let first = i / 10000;
        let second = (i / 1000) % 10;
        let third = (i / 100) % 10;
        let fourth = (i / 10) % 10;
        let fifth = i % 10;

        if first + third == 2 * second
            && second + fourth == 2 * third
            && third + fifth == 2 * fourth
        {
            cnt += 1;
        }
    }

    let precomputed = [
        123456, 234567, 345678, 456789, 1234567, 2345678, 3456789, 12345678, 23456789, 123456789,
        543210, 654321, 765432, 876543, 987654, 6543210, 7654321, 8765432, 9876543, 76543210,
        87654321, 98765432, 876543210, 987654321, 9876543210,
    ];

    for val in precomputed.iter() {
        if *val <= n {
            cnt += 1;
        }
    }

    let mut val = 111111;

    while val <= n {
        for i in 1..=9 {
            if val * i <= n {
                cnt += 1;
            }
        }

        val = val * 10 + 1;
    }

    writeln!(out, "{cnt}").unwrap();
}
