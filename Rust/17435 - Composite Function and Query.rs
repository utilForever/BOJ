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

    let m = scan.token::<usize>();
    let mut vals = vec![vec![0; 19]; 500001];

    for i in 1..=m {
        vals[i][0] = scan.token::<i64>();
    }

    for j in 1..19 {
        for i in 1..=m {
            vals[i][j] = vals[vals[i][j - 1] as usize][j - 1];
        }
    }

    let q = scan.token::<i64>();

    for _ in 0..q {
        let (n, mut x) = (scan.token::<i64>(), scan.token::<i64>());

        for i in (0..19).rev() {
            if n & (1 << i) != 0 {
                x = vals[x as usize][i];
            }
        }

        writeln!(out, "{x}").unwrap();
    }
}
