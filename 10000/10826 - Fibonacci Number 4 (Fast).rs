use io::Write;
use std::{cmp, io, str};

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

fn add_fibonacci(a: &mut Vec<i64>, b: &mut Vec<i64>) -> Vec<i64> {
    let mut c = Vec::new();
    let mut carry = 0;

    for i in 0..cmp::max(a.len(), b.len()) {
        let sum = carry + a.get(i).unwrap_or(&0) + b.get(i).unwrap_or(&0);
        c.push(sum % 10);
        carry = sum / 10;
    }

    if carry > 0 {
        c.push(carry);
    }

    c
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();

    if n == 0 {
        writeln!(out, "0").unwrap();
        return;
    }

    if n == 1 || n == 2 {
        writeln!(out, "1").unwrap();
        return;
    }

    let (mut a, mut b) = (vec![1], vec![1]);

    for _ in 3..=n {
        let c = add_fibonacci(&mut a, &mut b);
        (a, b) = (b, c);
    }

    for i in b.iter().rev() {
        write!(out, "{i}").unwrap();
    }

    writeln!(out).unwrap();
}
