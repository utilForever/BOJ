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

fn check1(intervals: &Vec<(i64, i64)>) -> bool {
    let n = intervals.len();
    let mut left = i64::MIN / 4;
    let mut right = i64::MAX / 4;

    for i in 0..n {
        if i > 0 {
            left -= 1;
            right += 1;
        }

        left = left.max(intervals[i].0);
        right = right.min(intervals[i].1);

        if left > right {
            return false;
        }
    }

    true
}

fn check2(intervals: &Vec<(i64, i64)>, skip: usize) -> bool {
    let n = intervals.len();
    let mut left = i64::MIN / 4;
    let mut right = i64::MAX / 4;

    for i in 0..n {
        if i > 0 {
            left -= 1;
            right += 1;
        }

        if i == skip {
            continue;
        }

        left = left.max(intervals[i].0);
        right = right.min(intervals[i].1);

        if left > right {
            return false;
        }
    }

    true
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut intervals = vec![(0, 0); n];

    for i in 0..n {
        intervals[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    if check1(&intervals) {
        writeln!(out, "World Champion").unwrap();
        return;
    }

    for i in 0..n {
        if check2(&intervals, i) {
            writeln!(out, "World Champion").unwrap();
            return;
        }
    }

    writeln!(out, "Surrender").unwrap();
}
