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

fn gcd_extended(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = gcd_extended(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, mut j) = (scan.token::<usize>(), scan.token::<usize>());
    let (a, b) = (scan.token::<i128>(), scan.token::<i128>());

    if a == 1 && b == 1 {
        writeln!(out, "-1").unwrap();
        return;
    }

    let mut prev_num: i128;
    let mut prev_den: i128;
    let mut curr_num: i128;
    let mut curr_den: i128;

    if a == 0 && b == 1 {
        if j == 0 {
            writeln!(out, "0 1").unwrap();
            return;
        }

        curr_num = 1;
        curr_den = n as i128;

        j -= 1;

        if j == 0 {
            writeln!(out, "{curr_num} {curr_den}").unwrap();
            return;
        }

        prev_num = 0;
        prev_den = 1;
    } else {
        let a_inv = gcd_extended(a, b).1;
        let mut q = a_inv;
        let k = (n as i128 - q) / b;

        q += k as i128 * b;

        let p = (a * q - 1) / b;

        prev_num = p;
        prev_den = q;
        curr_num = a;
        curr_den = b;
    }

    let mut rem = j;

    while rem > 0 {
        if curr_num == 1 && curr_den == 1 {
            writeln!(out, "-1").unwrap();
            return;
        }

        let k = (n as i128 + prev_den) / curr_den;
        let next_num = k * curr_num - prev_num;
        let next_den = k * curr_den - prev_den;

        prev_num = curr_num;
        prev_den = curr_den;
        curr_num = next_num;
        curr_den = next_den;

        rem -= 1;
    }

    writeln!(out, "{curr_num} {curr_den}").unwrap();
}
