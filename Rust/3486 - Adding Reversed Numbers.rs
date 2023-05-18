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

    let n = scan.token::<i64>();

    for _ in 0..n {
        let (a, b) = (scan.token::<String>(), scan.token::<String>());

        let mut a = a.chars().collect::<Vec<_>>();
        a.reverse();
        let a = a.iter().collect::<String>().parse::<i64>().unwrap();

        let mut b = b.chars().collect::<Vec<_>>();
        b.reverse();
        let b = b.iter().collect::<String>().parse::<i64>().unwrap();

        let ret = a + b;
        let mut ret = ret.to_string().chars().collect::<Vec<_>>();
        ret.reverse();
        let ret = ret.iter().collect::<String>().parse::<i64>().unwrap();

        writeln!(out, "{ret}").unwrap();
    }
}
