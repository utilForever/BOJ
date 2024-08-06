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

    for i in 1..=t {
        let (a, _, b, _, c) = (
            scan.token::<String>(),
            scan.token::<String>(),
            scan.token::<i64>(),
            scan.token::<String>(),
            scan.token::<i64>(),
        );
        let a_x_pos = a.find('x').unwrap();
        let a_multiplier = a[..a_x_pos].parse::<i64>().unwrap();

        writeln!(out, "Equation {i}").unwrap();

        if a_multiplier == 0 {
            writeln!(
                out,
                "{}",
                if b == c {
                    "More than one solution."
                } else {
                    "No solution."
                }
            )
            .unwrap();
        } else {
            let x = (c - b) as f64 / a_multiplier as f64;
            let x = x.to_string();
            let (a, b) = if x.contains('.') {
                let mut x = x.split('.');
                let a = x.next().unwrap();
                let b = x.next().unwrap();
                (a, b)
            } else {
                (x.as_str(), "0")
            };

            write!(out, "x = ").unwrap();

            if b.len() > 6 {
                writeln!(out, "{}.{}", a, &b[..6]).unwrap();
            } else if b.len() < 6 {
                write!(out, "{}.{}", a, b).unwrap();

                for _ in 0..6 - b.len() {
                    write!(out, "0").unwrap();
                }

                writeln!(out).unwrap();
            } else {
                writeln!(out, "{}.{}", a, b).unwrap();
            }
        }

        if i != t {
            writeln!(out).unwrap();
        }
    }
}
