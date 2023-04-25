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

    let (n, m, t, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut stones = vec![(0, 0); t];

    for i in 0..t {
        stones[i] = (scan.token::<usize>(), scan.token::<usize>());
    }

    let mut ret = 0;
    let mut pos = (0, 0);

    for stone1 in stones.iter() {
        for stone2 in stones.iter() {
            let mut cnt = 0;
            let x = if stone1.0 + k >= n { n - k } else { stone1.0 };
            let y = if stone2.1 + k >= m { m - k } else { stone2.1 };

            for stone in stones.iter() {
                if stone.0 >= x && stone.0 <= x + k && stone.1 >= y && stone.1 <= y + k {
                    cnt += 1;
                }
            }

            if cnt > ret {
                ret = cnt;
                pos = (x, y + k);
            }
        }
    }

    writeln!(out, "{} {}", pos.0, pos.1).unwrap();
    writeln!(out, "{ret}").unwrap();
}
