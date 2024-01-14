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

    let mut duration_min = 24 * 60 + 1;
    let mut duration_max = 0;

    for _ in 0..3 {
        let (h1, m1, h2, m2) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        let time1 = h1 * 60 + m1;
        let mut time2 = h2 * 60 + m2;

        if time2 < time1 {
            time2 += 24 * 60;
        }

        duration_min = duration_min.min(time2 - time1);
        duration_max = duration_max.max(time2 - time1);
    }

    writeln!(out, "{}:{:02}", duration_min / 60, duration_min % 60).unwrap();
    writeln!(out, "{}:{:02}", duration_max / 60, duration_max % 60).unwrap();
}
