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

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut qualities = vec![0; n];
    let mut multiplies = vec![0; n];

    for i in 0..n {
        qualities[i] = scan.token::<i64>();
    }

    for i in 0..n {
        multiplies[i] =
            qualities[i] * qualities[(i + 1) % n] * qualities[(i + 2) % n] * qualities[(i + 3) % n];
    }

    let mut sum = multiplies.iter().sum::<i64>();

    for _ in 0..q {
        let idx = scan.token::<usize>() - 1;

        for i in 0..4 {
            sum -= multiplies[(idx + n - i) % n];
            multiplies[(idx + n - i) % n] *= -1;
            sum += multiplies[(idx + n - i) % n];
        }

        writeln!(out, "{sum}").unwrap();
    }
}
