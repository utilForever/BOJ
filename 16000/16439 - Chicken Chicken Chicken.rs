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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut satisfactions = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            satisfactions[i][j] = scan.token::<i64>();
        }
    }

    let mut ret = 0;

    for i in 0..m {
        for j in 0..m {
            for k in 0..m {
                if i == j || j == k || i == k {
                    continue;
                }

                let mut val = 0;

                for l in 0..n {
                    val += satisfactions[l][i]
                        .max(satisfactions[l][j])
                        .max(satisfactions[l][k]);
                }

                ret = ret.max(val);
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}