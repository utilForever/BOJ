use io::Write;
use std::{cmp, io, str};

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
    let mut cnt = vec![vec![0; 2]; s.len() + 1];
    let mut idx = 1;

    let mut chars = s.chars();

    if chars.next().unwrap().is_uppercase() {
        cnt[0][0] = 2;
        cnt[0][1] = 2;
    } else {
        cnt[0][0] = 1;
        cnt[0][1] = 2;
    }

    loop {
        let c = chars.next();
        if c.is_none() {
            break;
        }

        cnt[idx][0] = cmp::min(
            cnt[idx - 1][0] + if c.unwrap().is_uppercase() { 1 } else { 0 },
            cnt[idx - 1][1] + 1,
        ) + 1;
        cnt[idx][1] = cmp::min(
            cnt[idx - 1][1] + if c.unwrap().is_lowercase() { 1 } else { 0 },
            cnt[idx - 1][0] + 1,
        ) + 1;

        idx += 1;
    }

    writeln!(out, "{}", cmp::min(cnt[idx - 1][0], cnt[idx - 1][1])).unwrap();
}
