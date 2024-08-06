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

    let (n, l, m) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut fishes = vec![(0, 0); m];

    for i in 0..m {
        fishes[i] = (scan.token::<usize>(), scan.token::<usize>());
    }

    let mut ret = 0;

    for fish1 in fishes.iter() {
        for fish2 in fishes.iter() {
            for width in (1..=(l - 2) / 2).rev() {
                let height = (l - 2 * width) / 2;

                let mut cnt = 0;
                let x = if fish1.0 + width >= n {
                    n - width
                } else {
                    fish1.0
                };
                let y = if fish2.1 + height >= n {
                    n - height
                } else {
                    fish2.1
                };

                for fish in fishes.iter() {
                    if fish.0 >= x && fish.0 <= x + width && fish.1 >= y && fish.1 <= y + height {
                        cnt += 1;
                    }
                }

                ret = ret.max(cnt);
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
