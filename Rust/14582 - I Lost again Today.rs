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

    let mut score_a = [0; 9];
    let mut score_b = [0; 9];

    for i in 0..9 {
        score_a[i] = scan.token::<i64>();
    }

    for i in 0..9 {
        score_b[i] = scan.token::<i64>();
    }

    let mut sum_a = 0;
    let mut sum_b = 0;
    let mut exist_win = false;

    for i in 0..9 {
        sum_a += score_a[i];

        if sum_a > sum_b {
            exist_win = true;
            break;
        }

        sum_b += score_b[i];
    }

    writeln!(out, "{}", if exist_win { "Yes" } else { "No" }).unwrap();
}
