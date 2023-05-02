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

    loop {
        let (n, width, length, height, area, m) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<usize>(),
        );

        if n == 0 && width == 0 && length == 0 && height == 0 && area == 0 && m == 0 {
            break;
        }

        let mut area_local = vec![(0, 0); m];

        for i in 0..m {
            area_local[i] = (scan.token::<i64>(), scan.token::<i64>());
        }

        let total = ((width * length + width * height * 2 + length * height * 2)
            - area_local.iter().fold(0, |acc, x| acc + x.0 * x.1))
            * n;

        writeln!(
            out,
            "{}",
            if total % area == 0 {
                total / area
            } else {
                total / area + 1
            }
        )
        .unwrap();
    }
}