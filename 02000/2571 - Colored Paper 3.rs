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

    let n = scan.token::<i64>();
    let mut paper = vec![vec![0; 101]; 101];

    for _ in 0..n {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());

        for i in a..a + 10 {
            for j in b..b + 10 {
                paper[i][j] = 1;
            }
        }
    }

    let mut prefix_sum = vec![vec![0; 101]; 101];

    for i in 1..101 {
        for j in 1..101 {
            prefix_sum[i][j] = paper[i][j] + prefix_sum[i - 1][j] + prefix_sum[i][j - 1]
                - prefix_sum[i - 1][j - 1];
        }
    }

    let mut ret = 0;

    for y1 in 1..100 {
        for x1 in 1..100 {
            for y2 in y1 + 1..101 {
                for x2 in x1 + 1..101 {
                    let sum = prefix_sum[y2][x2] - prefix_sum[y1 - 1][x2] - prefix_sum[y2][x1 - 1]
                        + prefix_sum[y1 - 1][x1 - 1];

                    if sum == (y2 - y1 + 1) * (x2 - x1 + 1) {
                        ret = ret.max(sum);
                    }
                }
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
