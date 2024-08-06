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

    let start = scan.token::<String>();
    let end = scan.token::<String>();

    let start = start.split(":").collect::<Vec<_>>();
    let start = start[0].parse::<i64>().unwrap() * 3600
        + start[1].parse::<i64>().unwrap() * 60
        + start[2].parse::<i64>().unwrap();
    let end = end.split(":").collect::<Vec<_>>();
    let end = end[0].parse::<i64>().unwrap() * 3600
        + end[1].parse::<i64>().unwrap() * 60
        + end[2].parse::<i64>().unwrap();

    let mut ret = end - start;

    if ret < 0 {
        ret += 24 * 3600;
    }

    writeln!(
        out,
        "{:02}:{:02}:{:02}",
        ret / 3600,
        (ret % 3600) / 60,
        ret % 60
    )
    .unwrap();
}
