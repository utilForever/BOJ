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

    let _ = scan.token::<i64>();
    let str = scan.token::<String>();
    let str = str.chars().collect::<Vec<_>>();

    let (mut prev_n, mut cur_n) = (0, 0);
    let (mut prev_s, mut cur_s) = (0, 0);
    let mut ret = 0;

    for c in str.iter() {
        if *c == 'N' {
            if cur_s > 0 {
                prev_s = cur_s;
                cur_s = 0;

                ret = ret.max(prev_n.min(prev_s));
            }

            cur_n += 1;
        } else {
            if cur_n > 0 {
                prev_n = cur_n;
                cur_n = 0;

                ret = ret.max(prev_n.min(prev_s));
            }

            cur_s += 1;
        }
    }

    ret = ret.max(prev_n.min(cur_s));
    ret = ret.max(cur_n.min(prev_s));

    writeln!(out, "{}", ret * 2).unwrap();
}
