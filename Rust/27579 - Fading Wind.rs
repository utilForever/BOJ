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

    let (h, k, v, s) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut velocity = v;
    let mut height = h;
    let mut strength = s;
    let mut ret = 0;

    while height > 0 {
        velocity += strength;
        velocity -= (velocity / 10).max(1);

        if velocity >= k {
            height += 1;
        } else if velocity > 0 && velocity < k {
            height -= 1;

            if height == 0 {
                velocity = 0;
            }
        } else {
            velocity = 0;
            height = 0;
        }

        ret += velocity;

        if strength > 0 {
            strength -= 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
