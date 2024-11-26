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

    let (a, op1, b, op2, c) = (
        scan.token::<f64>(),
        scan.token::<char>(),
        scan.token::<f64>(),
        scan.token::<char>(),
        scan.token::<f64>(),
    );
    let mut ret = 0.0;

    ret += match op1 {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        _ => unreachable!(),
    };

    ret = match op2 {
        '+' => ret + c,
        '-' => ret - c,
        '*' => ret * c,
        '/' => ret / c,
        _ => unreachable!(),
    };

    writeln!(out, "=================").unwrap();
    writeln!(out, "|SASA CALCULATOR|").unwrap();
    writeln!(out, "|{:>15.3}|", ret).unwrap();
    writeln!(out, "-----------------").unwrap();
    writeln!(out, "|               |").unwrap();
    writeln!(out, "| AC         /  |").unwrap();
    writeln!(out, "| 7  8  9    *  |").unwrap();
    writeln!(out, "| 4  5  6    -  |").unwrap();
    writeln!(out, "| 1  2  3    +  |").unwrap();
    writeln!(out, "|    0  .    =  |").unwrap();
    writeln!(out, "=================").unwrap();
}
