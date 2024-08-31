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

    let (n, t) = (scan.token::<usize>(), scan.token::<i64>());
    let mut buses = vec![(0, 0, 0); n];

    for i in 0..n {
        buses[i] = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
    }

    let mut departure = vec![0; n];

    for i in 0..n {
        let (start, interval, capacity) = buses[i];
        let mut time = start;
        let mut cnt = 1;

        while time < t {
            time += interval;
            cnt += 1;
        }

        departure[i] = if cnt > capacity { 0 } else { time };
    }

    let mut min = i64::MAX;

    for i in 0..n {
        if departure[i] < min && departure[i] != 0 {
            min = departure[i];
        }
    }

    writeln!(out, "{}", if min == i64::MAX { -1 } else { min - t }).unwrap();
}
