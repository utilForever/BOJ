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

fn build_failure(pat: &[u8]) -> Vec<usize> {
    let mut fail = vec![0; pat.len()];
    let mut j = 0;

    for i in 1..pat.len() {
        while j > 0 && pat[i] != pat[j] {
            j = fail[j - 1];
        }

        if pat[i] == pat[j] {
            j += 1;
            fail[i] = j;
        }
    }

    fail
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let z = scan.token::<i64>();

    for _ in 0..z {
        let (w, t) = (scan.token::<String>(), scan.token::<String>());
        let (w, t) = (w.as_bytes(), t.as_bytes());
        let fail = build_failure(w);
        let mut idx = 0;
        let mut ret = 0;

        for &c in t {
            while idx > 0 && c != w[idx] {
                idx = fail[idx - 1];
            }

            if c == w[idx] {
                idx += 1;

                if idx == w.len() {
                    ret += 1;
                    idx = fail[idx - 1];
                }
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
