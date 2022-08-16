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

    let n = scan.token::<usize>();
    let s = scan.token::<String>();
    let chars = s.chars().collect::<Vec<_>>();
    let mut ret = vec![vec![0_i64; 4]; n + 1];

    for i in 1..=n {
        for j in 0..4 {
            ret[i][j] = ret[i - 1][j];
        }

        if chars[i - 1] == 'D' {
            ret[i][0] += 1;
        } else if chars[i - 1] == 'K' {
            ret[i][1] += ret[i - 1][0];
        } else if chars[i - 1] == 'S' {
            ret[i][2] += ret[i - 1][1];
        } else if chars[i - 1] == 'H' {
            ret[i][3] += ret[i - 1][2];
        }
    }

    writeln!(out, "{}", ret[n][3]).unwrap();
}
