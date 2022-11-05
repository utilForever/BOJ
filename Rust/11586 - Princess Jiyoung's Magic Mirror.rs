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

    let n = scan.token::<usize>();
    let mut mirror = vec![vec![' '; n]; n];

    for i in 0..n {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            mirror[i][j] = c;
        }
    }

    let k = scan.token::<i64>();

    if k == 1 {
        for i in 0..n {
            for j in 0..n {
                write!(out, "{}", mirror[i][j]).unwrap();
            }

            writeln!(out).unwrap();
        }
    } else if k == 2 {
        for i in 0..n {
            for j in 0..n {
                write!(out, "{}", mirror[i][n - j - 1]).unwrap();
            }

            writeln!(out).unwrap();
        }
    } else if k == 3 {
        for i in 0..n {
            for j in 0..n {
                write!(out, "{}", mirror[n - i - 1][j]).unwrap();
            }

            writeln!(out).unwrap();
        }
    }
}
