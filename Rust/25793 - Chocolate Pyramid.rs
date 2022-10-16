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

    for _ in 0..t {
        let (r, c) = (scan.token::<i64>(), scan.token::<i64>());
        let height = c.min(r);

        let calculate_rc =
            height * r * c - height * (height - 1) / 2 * r - height * (height - 1) / 2 * c
                + (height - 1) * height * (2 * height - 1) / 6;
        let calculate_r = r * (r + 1) / 2 - (r - height) * (r - height + 1) / 2;
        let calculate_c = c * (c + 1) / 2 - (c - height) * (c - height + 1) / 2;
        let ret = 2 * calculate_rc - calculate_r - calculate_c;

        writeln!(out, "{} {}", ret + height, ret).unwrap();
    }
}
