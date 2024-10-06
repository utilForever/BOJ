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

    let mut keyboard = [[' '; 10]; 4];

    for i in 0..4 {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            keyboard[i][j] = c;
        }
    }

    let mut s = scan.token::<String>().chars().collect::<Vec<_>>();
    s.sort();

    for i in 1..3 {
        for j in 1..9 {
            let mut neighbors = vec![
                keyboard[i - 1][j - 1],
                keyboard[i - 1][j],
                keyboard[i - 1][j + 1],
                keyboard[i][j - 1],
                keyboard[i][j],
                keyboard[i][j + 1],
                keyboard[i + 1][j - 1],
                keyboard[i + 1][j],
                keyboard[i + 1][j + 1],
            ];
            neighbors.sort();

            if s == neighbors {
                writeln!(out, "{}", keyboard[i][j]).unwrap();
                return;
            }
        }
    }
}
