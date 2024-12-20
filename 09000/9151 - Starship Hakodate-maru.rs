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

    let mut fuel1 = Vec::new();
    let mut fuel2 = Vec::new();

    for i in 0..=53 {
        fuel1.push(i * i * i);
    }

    for i in 0..=95 {
        fuel2.push(i * (i + 1) * (i + 2) / 6);
    }

    loop {
        let n = scan.token::<i64>();

        if n == 0 {
            break;
        }

        let mut ret = 0;

        for &val1 in fuel1.iter() {
            for &val2 in fuel2.iter() {
                if val1 + val2 > n {
                    break;
                }

                ret = ret.max(val1 + val2);
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
