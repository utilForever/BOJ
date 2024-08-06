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

    let (h, w) = (scan.token::<usize>(), scan.token::<usize>());
    let mut residents = vec![vec![0; w]; h];

    for i in 0..h {
        for j in 0..w {
            residents[i][j] = scan.token::<i64>();
        }
    }

    let mut ret = i64::MAX;

    for i in 0..h {
        for j in 0..w {
            let mut cnt = 0;

            for k in 0..h {
                for l in 0..w {
                    let dist = (i as i64 - k as i64).abs().min((j as i64 - l as i64).abs());
                    cnt += residents[k][l] * dist;
                }
            }

            ret = ret.min(cnt);
        }
    }

    writeln!(out, "{ret}").unwrap();
}
