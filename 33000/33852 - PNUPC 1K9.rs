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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    let s = scan.token::<String>();
    let (k, p) = (scan.token::<i64>(), scan.token::<i64>());

    if s.len() == 1 && k >= 36 {
        writeln!(out, "-1").unwrap();
        return;
    }

    let s = s
        .chars()
        .map(|c| match c {
            '0'..='9' => (c as u8 - b'0') as i64,
            'A'..='Z' => (c as u8 - b'A' + 10) as i64,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    let mut pow = 1;
    let mut sum = 0;

    for &num in s.iter().rev() {
        sum = (sum + pow * num) % p;
        pow = (pow * 36) % p;
    }

    if sum == k {
        writeln!(out, "0").unwrap();
        return;
    }

    let mut pow = 1;

    for &num in s.iter().rev() {
        let val1 = sum - (pow * num) % p;

        for i in 0..=35 {
            let val2 = (((val1 + pow * i) % p) + p) % p;

            if val2 == k {
                writeln!(out, "1").unwrap();
                return;
            }
        }

        pow = (pow * 36) % p;
    }

    writeln!(out, "2").unwrap();
}
