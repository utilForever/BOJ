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

    let s = scan.token::<String>();
    let s = s.chars().collect::<Vec<_>>();

    let mut idx = 0;
    let is_a_minus = s[idx] == '-';

    if is_a_minus {
        idx += 1;
    }

    let mut a = String::new();

    while s[idx].is_numeric() {
        a.push(s[idx]);
        idx += 1;
    }

    let op = s[idx];
    idx += 1;

    let is_b_minus = s[idx] == '-';

    if is_b_minus {
        idx += 1;
    }

    let mut b = String::new();

    while idx < s.len() && s[idx].is_numeric() {
        b.push(s[idx]);
        idx += 1;
    }

    let a = i64::from_str_radix(&a, 8).unwrap() * if is_a_minus { -1 } else { 1 };
    let b = i64::from_str_radix(&b, 8).unwrap() * if is_b_minus { -1 } else { 1 };

    if op == '/' && b == 0 {
        writeln!(out, "invalid").unwrap();
        return;
    }

    match op {
        '+' => {
            if a + b < 0 {
                writeln!(out, "-{:o}", -(a + b)).unwrap();
            } else {
                writeln!(out, "{:o}", a + b).unwrap();
            }
        }
        '-' => {
            if a - b < 0 {
                writeln!(out, "-{:o}", -(a - b)).unwrap();
            } else {
                writeln!(out, "{:o}", a - b).unwrap();
            }
        }
        '*' => {
            if a * b < 0 {
                writeln!(out, "-{:o}", -(a * b)).unwrap();
            } else {
                writeln!(out, "{:o}", a * b).unwrap();
            }
        }
        '/' => {
            let ret = a.abs() / b.abs();
            let mut ret = format!("{:o}", ret).parse::<i128>().unwrap();

            if is_a_minus ^ is_b_minus {
                ret = !ret;

                if b.abs() == 1 {
                    ret += 1;
                }
            }

            writeln!(out, "{ret}").unwrap();
        }
        _ => unreachable!(),
    }
}
