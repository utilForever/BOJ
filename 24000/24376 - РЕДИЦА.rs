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

fn multiply(digits: &mut Vec<u8>, x: i64) {
    let mut carry = 0;

    for d in digits.iter_mut() {
        let prod = (*d as i64) * x + carry;

        *d = (prod % 10) as u8;
        carry = prod / 10;
    }

    while carry > 0 {
        digits.push((carry % 10) as u8);
        carry /= 10;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (x, n) = (scan.token::<i64>(), scan.token::<usize>());

    if n == 1 {
        writeln!(out, "1").unwrap();
        return;
    }

    let mut digits = vec![1];
    let mut sequence = Vec::with_capacity(n);

    sequence.push(b'1');

    while sequence.len() < n {
        multiply(&mut digits, x);

        for &d in digits.iter().rev() {
            sequence.push(d + b'0');

            if sequence.len() == n {
                break;
            }
        }
    }

    writeln!(out, "{}", sequence[n - 1] as char).unwrap();
}
