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

    let mut score_white = 0;
    let mut score_black = 0;

    for _ in 0..8 {
        let line = scan.token::<String>();

        for ch in line.chars() {
            match ch {
                'Q' => score_white += 9,
                'R' => score_white += 5,
                'B' => score_white += 3,
                'N' => score_white += 3,
                'P' => score_white += 1,
                'q' => score_black += 9,
                'r' => score_black += 5,
                'b' => score_black += 3,
                'n' => score_black += 3,
                'p' => score_black += 1,
                _ => (),
            }
        }
    }

    writeln!(out, "{}", score_white - score_black).unwrap();
}
