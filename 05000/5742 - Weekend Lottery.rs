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

    loop {
        let (n, c, k) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<usize>(),
        );

        if n == 0 && c == 0 && k == 0 {
            break;
        }

        let mut records = vec![0; k + 1];

        for _ in 0..n {
            for _ in 0..c {
                let x = scan.token::<usize>();
                records[x] += 1;
            }
        }

        let mut ret = Vec::new();
        let mut min = i64::MAX;

        for i in 1..=k {
            if records[i] < min {
                min = records[i];
                ret.clear();
                ret.push(i);
            } else if records[i] == min {
                ret.push(i);
            }
        }

        for val in ret {
            write!(out, "{val} ").unwrap();
        }

        writeln!(out).unwrap();
    }
}
