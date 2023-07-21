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

    let n = scan.token::<i64>();
    let mut max_x1 = i64::MIN;
    let mut min_x2 = i64::MAX;
    let mut max_y1 = i64::MIN;
    let mut min_y2 = i64::MAX;

    for _ in 0..n {
        let (x1, x2, y1, y2) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        max_x1 = max_x1.max(x1);
        min_x2 = min_x2.min(x2);
        max_y1 = max_y1.max(y1);
        min_y2 = min_y2.min(y2);
    }

    writeln!(
        out,
        "{}",
        (min_x2 - max_x1).max(0) * (min_y2 - max_y1).max(0)
    )
    .unwrap();
}
