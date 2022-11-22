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

    loop {
        let k = scan.token::<i64>();

        if k == 0 {
            break;
        }

        let (n, m) = (scan.token::<i64>(), scan.token::<i64>());

        for _ in 0..k {
            let (x, y) = (scan.token::<i64>(), scan.token::<i64>());

            if n == x || m == y {
                writeln!(out, "divisa").unwrap();
            } else {
                write!(out, "{}", if y > m { "N" } else { "S" }).unwrap();
                writeln!(out, "{}", if x > n { "E" } else { "O" }).unwrap();
            }
        }
    }
}
