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

    let s = scan.token::<String>();

    if s.len() == 1 {
        writeln!(out, "0").unwrap();
        return;
    }

    let s = s.chars().collect::<Vec<_>>();
    let n = s.len();
    let mut ret = 0;

    for i in 0..n {
        let mut cnt = 1;

        while i + cnt * 2 <= n {
            let val_left = s[i..i + cnt].iter().map(|&c| c as i64).sum::<i64>();
            let val_right = s[i + cnt..i + cnt * 2]
                .iter()
                .map(|&c| c as i64)
                .sum::<i64>();

            if val_left == val_right {
                ret = ret.max(cnt * 2);
            }

            cnt += 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
