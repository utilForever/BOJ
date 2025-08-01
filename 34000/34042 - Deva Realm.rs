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

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());

    for _ in 0..m {
        let mut positives = Vec::with_capacity(n);
        let mut negatives = Vec::with_capacity(n);
        let mut has_zero = false;

        for _ in 0..n {
            let num = scan.token::<i64>();

            if num > 0 {
                positives.push(num);
            } else if num < 0 {
                negatives.push(num);
            } else {
                has_zero = true;
            }
        }

        positives.sort_unstable_by(|a, b| b.cmp(a));
        negatives.sort_unstable();

        if positives.is_empty() && negatives.is_empty() {
            writeln!(out, "0").unwrap();
        } else if positives.is_empty() && negatives.len() == 1 {
            writeln!(out, "{}", if has_zero { 0 } else { negatives[0] }).unwrap();
        } else {
            let mut ret = 1;

            for val in positives {
                ret *= val;
            }

            let mut idx = 0;

            while idx + 1 < negatives.len() {
                ret *= negatives[idx] * negatives[idx + 1];
                idx += 2;
            }

            writeln!(out, "{}", if has_zero { ret.max(0) } else { ret }).unwrap();
        }
    }
}
