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

static MOD: i64 = 1_000_000_007;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let s = scan.token::<String>();
        let mut cnt_h = 0;
        let mut cnt_q = 0;

        for c in s.chars() {
            if c == 'H' {
                cnt_h += 1;
            } else if c == '?' {
                cnt_q += 1;
            }
        }

        if cnt_q == 0 {
            writeln!(out, "{}", if cnt_h % 2 == 1 { 1 } else { 0 }).unwrap();
            continue;
        }

        let mut ret = 1;

        for _ in 0..cnt_q - 1 {
            ret = (ret * 2) % MOD;
        }

        writeln!(out, "{ret}").unwrap();
    }
}
