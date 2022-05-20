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
    let mut steps = vec![vec![0_i64; 10]; n + 1];

    for i in 1..=9 {
        steps[1][i] = 1;
    }

    for i in 2..=n {
        for j in 0..=9 {
            if j >= 1 {
                steps[i][j] += steps[i - 1][j - 1];
            }

            if j <= 8 {
                steps[i][j] += steps[i - 1][j + 1];
            }

            steps[i][j] %= 1_000_000_000;
        }
    }

    let mut ret = 0;

    for i in 0..=9 {
        ret += steps[n][i];
    }

    ret %= 1_000_000_000;

    writeln!(out, "{}", ret).unwrap();
}
