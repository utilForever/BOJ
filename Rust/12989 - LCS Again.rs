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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let s = scan.token::<String>();
    let s = s.chars().collect::<Vec<_>>();
    let mut ret = n * (m - 1);

    for i in 1..n {
        if s[i] != s[i - 1] {
            ret += n * (m - 1);
        }
    }

    let mut cnt = 0;

    for i in 1..n {
        if s[i] != s[i - 1] {
            if i != 1 && s[i] != s[i - 2] {
                cnt = 1;
            } else {
                cnt += 1;
            }
        } else {
            cnt = 0;
        }

        ret -= cnt;
    }

    writeln!(out, "{ret}").unwrap();
}
