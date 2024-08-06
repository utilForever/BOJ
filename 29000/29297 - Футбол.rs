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

    let n = scan.token::<i64>();

    for _ in 0..n {
        let score = scan.token::<String>();
        let score = score.chars().collect::<Vec<_>>();
        let score_lag = score[0].to_digit(10).unwrap();
        let score_dcu = score[2].to_digit(10).unwrap();
        let mut ret_lag = 0;
        let mut ret_dcu = 0;

        for i in 0..10 {
            for j in 0..10 {
                if score_lag + i > score_dcu + j {
                    ret_lag += 1;
                } else if score_lag + i < score_dcu + j {
                    ret_dcu += 1;
                } else if i > score_dcu {
                    ret_lag += 1;
                } else if i < score_dcu {
                    ret_dcu += 1;
                }
            }
        }

        writeln!(out, "{ret_lag} {ret_dcu}").unwrap();
    }
}
