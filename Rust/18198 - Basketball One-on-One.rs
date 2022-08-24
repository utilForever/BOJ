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

    let s = scan.token::<String>();
    let s = s.chars().collect::<Vec<_>>();

    let mut score_a = 0;
    let mut score_b = 0;
    let mut is_tied = false;

    for i in (0..s.len()).step_by(2) {
        if s[i] == 'A' {
            score_a += s[i + 1].to_digit(10).unwrap() as i64;
        } else {
            score_b += s[i + 1].to_digit(10).unwrap() as i64;
        }

        if score_a == 10 && score_b == 10 {
            is_tied = true;
        }

        if is_tied {
            if score_a - score_b >= 2 {
                writeln!(out, "A").unwrap();
                return;
            } else if score_b - score_a >= 2 {
                writeln!(out, "B").unwrap();
                return;
            }
        } else {
            if score_a >= 11 {
                writeln!(out, "A").unwrap();
                return;
            } else if score_b >= 11 {
                writeln!(out, "B").unwrap();
                return;
            }
        }
    }
}
