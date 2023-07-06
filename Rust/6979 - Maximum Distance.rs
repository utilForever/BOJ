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

    let t = scan.token::<i64>();

    for i in 1..=t {
        let n = scan.token::<usize>();
        let mut x = vec![0; n];
        let mut y = vec![0; n];

        for j in 0..n {
            x[j] = scan.token::<i64>();
        }

        for j in 0..n {
            y[j] = scan.token::<i64>();
        }

        let mut idx_x = 0;
        let mut idx_y = 0;
        let mut ret = 0;

        while idx_x < n && idx_y < n {
            if y[idx_y] >= x[idx_x] {
                ret = ret.max(idx_y as i64 - idx_x as i64);
                idx_y += 1;
            } else {
                idx_x += 1;
            }
        }

        writeln!(out, "The maximum distance is {ret}").unwrap();

        if i != t {
            writeln!(out).unwrap();
        }
    }
}
