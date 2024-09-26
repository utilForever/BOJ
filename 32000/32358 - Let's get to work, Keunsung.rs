use io::Write;
use std::{collections::BTreeSet, io, str};

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

    let n = scan.token::<i64>();
    let mut trashes = BTreeSet::new();
    let mut pos = 0;
    let mut ret = 0;

    for _ in 0..n {
        let command = scan.token::<i64>();

        if command == 1 {
            let x = scan.token::<i64>();
            trashes.insert(x);
        } else {
            while !trashes.is_empty() {
                let lower = trashes.range(..=pos).next_back().cloned();
                let upper = trashes.range(pos..).next().cloned();

                match (lower, upper) {
                    (Some(lower), Some(upper)) => {
                        let dist_lower = (pos - lower).abs();
                        let dist_upper = (upper - pos).abs();

                        if dist_lower <= dist_upper {
                            ret += dist_lower;
                            pos = lower;

                            trashes.remove(&lower);
                        } else {
                            ret += dist_upper;
                            pos = upper;

                            trashes.remove(&upper);
                        }
                    }
                    (Some(lower), None) => {
                        ret += (pos - lower).abs();
                        pos = lower;

                        trashes.remove(&lower);
                    }
                    (None, Some(upper)) => {
                        ret += (upper - pos).abs();
                        pos = upper;

                        trashes.remove(&upper);
                    }
                    (None, None) => break,
                }
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
