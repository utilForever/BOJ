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
    let mut lengths = vec![0; n];

    for i in 0..n {
        lengths[i] = scan.token::<i64>();
    }

    lengths.sort_by(|a, b| b.cmp(a));

    let mut idx = 1;
    let mut cnt = 0;
    let mut length = 0;
    let mut ret = 0;

    while idx < n {
        if lengths[idx] == lengths[idx - 1] || lengths[idx] == (lengths[idx - 1] - 1) {
            if cnt == 1 {
                cnt = 0;
                ret += length * lengths[idx];
            } else {
                cnt = 1;
                length = lengths[idx];
            }

            idx += 1;
        }

        idx += 1;
    }

    writeln!(out, "{ret}").unwrap();
}
