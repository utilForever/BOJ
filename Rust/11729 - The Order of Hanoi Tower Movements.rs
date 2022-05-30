use io::Write;
use std::{
    io::{self, BufWriter, StdoutLock},
    str,
};

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

fn process(out: &mut BufWriter<StdoutLock>, n: u32, from: u32, to: u32) {
    if n == 0 {
        return;
    }

    let remain = 6 - from - to;

    process(out, n - 1, from, remain);
    writeln!(out, "{from} {to}").unwrap();
    process(out, n - 1, remain, to);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<u32>();
    let ret = 2_i64.pow(n) - 1;

    writeln!(out, "{ret}").unwrap();
    process(&mut out, n, 1, 3);
}
