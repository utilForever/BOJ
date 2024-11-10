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

    let (sx, sy) = (scan.token::<i64>(), scan.token::<i64>());
    let (ex, ey) = (scan.token::<i64>(), scan.token::<i64>());
    let (px, py) = (scan.token::<i64>(), scan.token::<i64>());

    writeln!(
        out,
        "{}",
        if sx == ex {
            if px == sx && ((py > sy && py < ey) || (py > ey && py < sy)) {
                2
            } else {
                0
            }
        } else if sy == ey {
            if py == sy && ((px > sx && px < ex) || (px > ex && px < sx)) {
                2
            } else {
                0
            }
        } else if !((py == sy && ((px > sx && px < ex) || (px > ex && px < sx)))
            || (px == ex && ((py > sy && py < ey) || (py > ey && py < sy))))
        {
            1
        } else if !((px == sx && ((py > sy && py < ey) || (py > ey && py < sy)))
            || (py == ey && ((px > sx && px < ex) || (px > ex && px < sx))))
        {
            1
        } else {
            2
        }
    )
    .unwrap();
}
