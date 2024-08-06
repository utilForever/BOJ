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

    let (h, m, s) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut time = h * 3600 + m * 60 + s;

    let q = scan.token::<i64>();

    for _ in 0..q {
        let t = scan.token::<i64>();

        if t == 1 {
            let c = scan.token::<i64>();

            time = (time + c) % (24 * 3600);
        } else if t == 2 {
            let c = scan.token::<i64>();

            time = (time - c) % (24 * 3600);

            if time < 0 {
                time += 24 * 3600;
            }
        } else if t == 3 {
            writeln!(out, "{} {} {}", time / 3600, time % 3600 / 60, time % 60).unwrap();
        }
    }
}
