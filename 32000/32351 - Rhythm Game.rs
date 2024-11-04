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

    let (n, mut bpm_curr, k) = (
        scan.token::<i64>(),
        scan.token::<f64>(),
        scan.token::<i64>(),
    );
    let mut bar_curr = 1;
    let mut ret = 0.0;

    for _ in 0..k {
        let (bar, bpm) = (scan.token::<i64>(), scan.token::<f64>());

        ret += (bar - bar_curr) as f64 * 4.0 * (60.0 / bpm_curr);
        bar_curr = bar;
        bpm_curr = bpm;
    }

    ret += (n - bar_curr + 1) as f64 * 4.0 * (60.0 / bpm_curr);

    writeln!(out, "{:.12}", ret).unwrap();
}
