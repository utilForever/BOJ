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

    let (n, k) = (scan.token::<i64>(), scan.token::<usize>());
    let mut scores = vec![0; k];

    for i in 0..k {
        scores[i] = scan.token::<i64>();
    }

    for score in scores {
        let percent = (score as f64 / n as f64 * 100.0) as i64;
        write!(
            out,
            "{} ",
            if percent <= 4 {
                1
            } else if percent <= 11 {
                2
            } else if percent <= 23 {
                3
            } else if percent <= 40 {
                4
            } else if percent <= 60 {
                5
            } else if percent <= 77 {
                6
            } else if percent <= 89 {
                7
            } else if percent <= 96 {
                8
            } else {
                9
            }
        )
        .unwrap();
    }
}
