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

fn precompute_euler_zigzag() -> Vec<i64> {
    // Entringer triangle
    let mut t = [[0; 21]; 21];
    t[0][0] = 1;

    for n in 1..=20 {
        t[n][0] = 0;

        for k in 1..=n {
            t[n][k] = t[n][k - 1] + t[n - 1][n - k];
        }
    }

    // Euler zigzag number
    let mut e = vec![0; 21];

    for n in 0..=20 {
        e[n] = t[n][n];
    }

    e
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let euler_zigzag = precompute_euler_zigzag();
    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        writeln!(out, "{}", if n == 1 { 1 } else { euler_zigzag[n] * 2 }).unwrap();
    }
}
