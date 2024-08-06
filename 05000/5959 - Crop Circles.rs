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
    let mut circles = vec![(0, 0, 0); n];

    for i in 0..n {
        circles[i] = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
    }

    let mut ret = vec![0; n];

    for i in 0..n {
        let mut cnt = 0;

        for j in 0..n {
            if i == j {
                continue;
            }

            let (x1, y1, r1) = circles[i];
            let (x2, y2, r2) = circles[j];

            let dist = (x1 - x2).pow(2) + (y1 - y2).pow(2);

            if dist <= (r1 + r2).pow(2) {
                cnt += 1;
            }
        }

        ret[i] = cnt;
    }

    for val in ret {
        writeln!(out, "{val}").unwrap();
    }
}
