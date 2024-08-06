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

const MOD: i64 = 1_000_000_009;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let s = scan.token::<String>();
    let s = s.chars().collect::<Vec<_>>();
    let mut ret = if s[0] == 'd' { 10 } else { 26 };

    for i in 1..s.len() {
        if s[i] == 'd' {
            if s[i - 1] == 'd' {
                ret = (ret * 9) % MOD;
            } else {
                ret = (ret * 10) % MOD;
            }
        } else {
            if s[i - 1] == 'c' {
                ret = (ret * 25) % MOD;
            } else {
                ret = (ret * 26) % MOD;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
