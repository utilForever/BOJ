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

    let (n, m, x, y) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let (_, h) = (scan.token::<i64>(), scan.token::<i64>());
    let mut scores = Vec::with_capacity(n);

    for _ in 0..n - 1 {
        let (num, s) = (scan.token::<String>(), scan.token::<i64>());

        if num.starts_with("2024") {
            let diff = x - s;
            let sum = s + (y - diff).max(0);
            scores.push(sum);
        }
    }

    scores.sort_by(|a, b| b.cmp(&a));

    if m > scores.len() {
        writeln!(out, "YES").unwrap();
        writeln!(out, "0").unwrap();
        return;
    }

    let mut idx = 0;

    for i in 1..scores.len() {
        if scores[i - 1] > scores[i] {
            if i >= m {
                break;
            }

            idx = i;
        }
    }

    if scores[idx] <= h + y {
        writeln!(out, "YES").unwrap();
        writeln!(
            out,
            "{}",
            if scores[idx] - h > 0 {
                scores[idx] - h
            } else {
                0
            }
        )
        .unwrap();
    } else {
        writeln!(out, "NO").unwrap();
    }
}
