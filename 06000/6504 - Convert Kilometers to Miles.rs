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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    let t = scan.token::<i64>();
    let mut fibonacci = vec![1, 2];

    while *fibonacci.last().unwrap() <= 50_000 {
        let n = fibonacci.len();
        fibonacci.push(fibonacci[n - 1] + fibonacci[n - 2]);
    }

    for _ in 0..t {
        let mut x = scan.token::<i64>();
        let mut y = 0;
        let mut idx = fibonacci.len() - 1;

        while x > 0 {
            if fibonacci[idx] <= x {
                x -= fibonacci[idx];

                if idx > 0 {
                    y += fibonacci[idx - 1];
                }

                if idx >= 2 {
                    idx -= 2;
                } else {
                    break;
                }
            } else {
                if idx == 0 {
                    break;
                }

                idx -= 1;
            }
        }

        writeln!(out, "{y}").unwrap();
    }
}
