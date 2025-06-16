use io::Write;
use std::{io, str, vec};

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

fn floor_sum_ap(a: i64, b: i64, c: i64, n: i64) -> i64 {
    if a == 0 {
        return (b / c) * (n + 1);
    }

    if a >= c || b >= c {
        return ((n * (n + 1)) / 2) * (a / c)
            + (n + 1) * (b / c)
            + floor_sum_ap(a % c, b % c, c, n);
    }

    let m = (a * n + b) / c;

    m * n - floor_sum_ap(c, c - b - 1, a, m - 1)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let flip = if n % 2 == 0 { 1 } else { 0 };
    let mut parity = vec![0; n + 1];

    for i in 0..=n {
        let h = floor_sum_ap(i as i64, 0, n as i64, n as i64);
        parity[i] = if h % 2 == 0 { 0 } else { 1 };
    }

    for i in 0..n {
        let h = parity[i] ^ parity[i + 1];
        write!(out, "{}", if h ^ flip == 1 { 'V' } else { '.' }).unwrap();
    }
}
