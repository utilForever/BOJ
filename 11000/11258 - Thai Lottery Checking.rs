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

    let (number, amount) = (scan.token::<String>(), scan.token::<i64>());
    let prize1 = (scan.token::<String>(), scan.token::<i64>());
    let prize2 = (scan.token::<String>(), scan.token::<i64>());
    let prize3 = (scan.token::<String>(), scan.token::<i64>());
    let prize4 = (scan.token::<String>(), scan.token::<i64>());
    let prize5 = (scan.token::<String>(), scan.token::<i64>());

    loop {
        let num = scan.token::<String>();

        if num == "-1" {
            break;
        }

        let mut ret = 0;

        if num == number {
            ret += amount;
        }

        if num.starts_with(&prize1.0) {
            ret += prize1.1;
        }

        if num.starts_with(&prize2.0) {
            ret += prize2.1;
        }

        if num.ends_with(&prize3.0) {
            ret += prize3.1;
        }

        if num.ends_with(&prize4.0) {
            ret += prize4.1;
        }

        if num.ends_with(&prize5.0) {
            ret += prize5.1;
        }

        writeln!(out, "{ret}").unwrap();
    }
}
