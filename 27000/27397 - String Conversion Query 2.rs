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

    let (s, n) = (scan.token::<String>(), scan.token::<i64>());
    let s = s.chars().collect::<Vec<_>>();
    let mut alphabets = vec![0; 52];

    for i in 0..26 {
        alphabets[i] = i + 'a' as usize;
        alphabets[i + 26] = i + 'A' as usize;
    }

    for _ in 0..n {
        let num = scan.token::<i64>();

        if num == 1 {
            let (a, b) = (scan.token::<String>(), scan.token::<String>());
            let (a, b) = (a.chars().next().unwrap(), b.chars().next().unwrap());

            for i in 0..52 {
                if alphabets[i] == a as usize {
                    alphabets[i] = b as usize;
                }
            }
        } else {
            let mut cnt = 1;
            let mut ret = 1;

            for i in 1..s.len() {
                let idx1 = if s[i].is_lowercase() {
                    s[i] as usize - 'a' as usize
                } else {
                    s[i] as usize - 'A' as usize + 26
                };
                let idx2 = if s[i - 1].is_lowercase() {
                    s[i - 1] as usize - 'a' as usize
                } else {
                    s[i - 1] as usize - 'A' as usize + 26
                };

                if alphabets[idx1] == alphabets[idx2] {
                    cnt += 1;
                } else {
                    cnt = 1;
                }

                ret = ret.max(cnt);
            }

            writeln!(out, "{ret}").unwrap();
        }
    }
}
