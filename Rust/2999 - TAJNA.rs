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

    let s = scan.token::<String>();
    let s = s.chars().collect::<Vec<_>>();
    let n = s.len();

    let mut r = 1;
    let mut c = 1;
    let mut val = 1;
    let mut limit = (n as f64).sqrt() as usize;

    if limit * limit < n {
        limit += 1;
    }

    while val <= limit {
        if n % val == 0 {
            r = val;
            c = n / val;
        }

        val += 1;
    }

    if r < c {
        std::mem::swap(&mut r, &mut c);
    }

    let mut ret = vec![vec![' '; c]; r];

    for i in 0..r {
        for j in 0..c {
            ret[i][j] = s[i * c + j];
        }
    }

    for i in 0..c {
        for j in 0..r {
            write!(out, "{}", ret[j][i]).unwrap();
        }
    }

    writeln!(out).unwrap();
}
