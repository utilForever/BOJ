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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
    let time_curr = x * 60 + y;

    let n = scan.token::<i64>();
    let mut wait_min = i64::MAX;

    for _ in 0..n {
        let (x, y, d) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        let time_start = x * 60 + y;
        let time_wait = if time_curr <= time_start {
            time_start - time_curr
        } else {
            let diff = time_curr - time_start;
            let cnt = (diff + d - 1) / d;
            let start_next = time_start + cnt * d;

            if start_next >= 24 * 60 {
                time_start + (24 * 60) - time_curr
            } else {
                start_next - time_curr
            }
        };

        wait_min = wait_min.min(time_wait);
    }

    let ret = (time_curr + wait_min) % 1440;

    writeln!(out, "{:02}:{:02}", ret / 60, ret % 60).unwrap();
}
