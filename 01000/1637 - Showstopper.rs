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

fn count_upto(x: u64, sequences: &Vec<(u64, u64, u64)>) -> u64 {
    let mut ret = 0;

    for &(a, c, b) in sequences {
        if x < a {
            continue;
        }

        let limit = if x < c { x } else { c };

        if limit >= a {
            ret += (limit - a) / b + 1;
        }
    }

    ret
}

fn count_exact(x: u64, sequences: &Vec<(u64, u64, u64)>) -> u64 {
    let mut ret = 0;

    for &(a, c, b) in sequences {
        if a <= x && x <= c && (x - a) % b == 0 {
            ret += 1;
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut sequences = vec![(0, 0, 0); n];

    for i in 0..n {
        sequences[i] = (
            scan.token::<u64>(),
            scan.token::<u64>(),
            scan.token::<u64>(),
        );
    }

    let mut left = 1;
    let mut right = *sequences.iter().map(|(_, c, _)| c).max().unwrap();

    while left < right {
        let mid = (left + right) / 2;
        let cnt = count_upto(mid, &sequences);

        if cnt % 2 == 1 {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    let ret = count_exact(left, &sequences);

    if ret % 2 == 1 {
        writeln!(out, "{left} {ret}").unwrap();
    } else {
        writeln!(out, "NOTHING").unwrap();
    }
}
