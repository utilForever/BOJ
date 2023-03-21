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

    let (n, k) = (scan.token::<i64>(), scan.token::<i64>());
    let mut students = [[0; 2]; 6];

    for _ in 0..n {
        let (s, y) = (scan.token::<usize>(), scan.token::<usize>());
        students[y - 1][s] += 1;
    }

    let mut ret = 0;

    for i in (0..6).step_by(2) {
        if i == 0 {
            let sum = students[i][0] + students[i][1] + students[i + 1][0] + students[i + 1][1];

            ret += if sum % k == 0 { sum / k } else { sum / k + 1 };
        } else {
            for j in 0..2 {
                let sum = students[i][j] + students[i + 1][j];

                ret += if sum % k == 0 { sum / k } else { sum / k + 1 };
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
