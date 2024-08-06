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

    let n = scan.token::<i64>();
    let mut mid = Vec::new();
    let mut score_ciel = 0;
    let mut score_jiro = 0;

    for _ in 0..n {
        let s = scan.token::<i64>();

        if s % 2 == 0 {
            for i in 0..s {
                let c = scan.token::<i64>();

                if i < s / 2 {
                    score_ciel += c;
                } else {
                    score_jiro += c;
                }
            }
        } else {
            for i in 0..s {
                let c = scan.token::<i64>();

                if i == s / 2 {
                    mid.push(c);
                } else if i < s / 2 {
                    score_ciel += c;
                } else {
                    score_jiro += c;
                }
            }
        }
    }

    mid.sort_by(|a, b| b.cmp(a));

    for i in 0..mid.len() {
        if i % 2 == 0 {
            score_ciel += mid[i];
        } else {
            score_jiro += mid[i];
        }
    }

    writeln!(out, "{score_ciel} {score_jiro}").unwrap();
}
