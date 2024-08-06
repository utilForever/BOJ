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

fn calculate_dist(x: i64, y: i64, a: i64, b: i64) -> i64 {
    (x - a).pow(2) + (y - b).pow(2)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (w, h, x, y, p) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let r = (h / 2).pow(2);
    let mut ret = 0;

    for _ in 0..p {
        let (p_x, p_y) = (scan.token::<i64>(), scan.token::<i64>());

        if p_x >= x && p_x <= x + w && p_y >= y && p_y <= y + h {
            ret += 1;
        } else if calculate_dist(x, y + h / 2, p_x, p_y) <= r {
            ret += 1;
        } else if calculate_dist(x + w, y + h / 2, p_x, p_y) <= r {
            ret += 1;
        }
    }

    writeln!(out, "{}", ret).unwrap();
}
