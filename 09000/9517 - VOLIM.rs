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

    let mut k = scan.token::<i64>() - 1;
    let n = scan.token::<usize>();
    let mut times = vec![0; n];
    let mut results = vec![' '; n];
    let mut total_t = 0;

    for i in 0..n {
        (times[i], results[i]) = (scan.token::<i64>(), scan.token::<char>());
    }

    for i in 0..n {
        if total_t + times[i] >= 210 {
            writeln!(out, "{}", k + 1).unwrap();
            break;
        }

        if results[i] == 'T' {
            k = (k + 1) % 8;
        }

        total_t += times[i];
    }
}
