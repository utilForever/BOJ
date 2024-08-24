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

    let mut n = scan.token::<i64>();
    let mut prev = i64::MAX;

    loop {
        let mut factorial = 1;
        let mut num = 0;

        loop {
            factorial *= if num == 0 { 1 } else { num };

            if factorial * (num + 1) > n || num + 1 >= prev {
                break;
            }

            num += 1;
        }

        if num == prev {
            writeln!(out, "NO").unwrap();
            return;
        }

        n -= factorial;
        prev = num;

        if n <= 0 {
            break;
        }
    }

    writeln!(out, "{}", if n == 0 { "YES" } else { "NO" }).unwrap();
}
