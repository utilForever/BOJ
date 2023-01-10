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
    let mut paper = vec![vec![0; 101]; 101];

    for _ in 0..n {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());

        for i in a..a + 10 {
            for j in b..b + 10 {
                paper[i][j] = 1;
            }
        }
    }

    let dx: [i64; 4] = [1, 0, -1, 0];
    let dy: [i64; 4] = [0, 1, 0, -1];
    let mut ret = 0;

    for i in 1..=100 {
        for j in 1..=100 {
            if paper[i][j] == 1 {
                for k in 0..4 {
                    let (next_x, next_y) = (i as i64 + dx[k], j as i64 + dy[k]);

                    if paper[next_x as usize][next_y as usize] == 0 {
                        ret += 1;
                    }
                }
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
