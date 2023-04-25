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

    let (_, _, l, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut stars = vec![(0, 0); k];

    for i in 0..k {
        stars[i] = (scan.token::<usize>(), scan.token::<usize>());
    }

    let mut ret = 0;

    for star1 in stars.iter() {
        for star2 in stars.iter() {
            let mut cnt = 0;
            let x = star1.0;
            let y = star2.1;

            for star in stars.iter() {
                if star.0 >= x && star.0 <= x + l && star.1 >= y && star.1 <= y + l {
                    cnt += 1;
                }
            }

            ret = ret.max(cnt);
        }
    }

    writeln!(out, "{}", k - ret).unwrap();
}
