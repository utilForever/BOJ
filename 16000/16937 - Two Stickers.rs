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

    let (h, w, n) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<usize>(),
    );
    let mut stickers = vec![(0, 0); n];

    for i in 0..n {
        stickers[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let mut area = 0;
    let mut ret = 0;

    for i in 0..n - 1 {
        for j in i + 1..n {
            let cond1 = (stickers[i].0 + stickers[j].0 <= h
                && stickers[i].1.max(stickers[j].1) <= w)
                || (stickers[i].0 + stickers[j].0 <= w && stickers[i].1.max(stickers[j].1) <= h);
            let cond2 = (stickers[i].0 + stickers[j].1 <= h
                && stickers[i].1.max(stickers[j].0) <= w)
                || (stickers[i].0 + stickers[j].1 <= w && stickers[i].1.max(stickers[j].0) <= h);
            let cond3 = (stickers[i].1 + stickers[j].0 <= h
                && stickers[i].0.max(stickers[j].1) <= w)
                || (stickers[i].1 + stickers[j].0 <= w && stickers[i].0.max(stickers[j].1) <= h);
            let cond4 = (stickers[i].1 + stickers[j].1 <= h
                && stickers[i].0.max(stickers[j].0) <= w)
                || (stickers[i].1 + stickers[j].1 <= w && stickers[i].0.max(stickers[j].0) <= h);

            if cond1 || cond2 || cond3 || cond4 {
                area = stickers[i].0 * stickers[i].1 + stickers[j].0 * stickers[j].1;
            }

            ret = ret.max(area);
        }
    }

    writeln!(out, "{ret}").unwrap();
}
