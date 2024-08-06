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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut relationships = vec![vec![0; n + 1]; n + 1];

    for _ in 0..m {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());
        relationships[a][b] = 1;
        relationships[b][a] = 1;
    }

    for k in 1..=n {
        for i in 1..=n {
            for j in 1..=n {
                if i == j {
                    continue;
                }

                if relationships[i][k] != 0 && relationships[k][j] != 0 {
                    if relationships[i][j] == 0 {
                        relationships[i][j] = relationships[i][k] + relationships[k][j];
                    } else {
                        relationships[i][j] = std::cmp::min(
                            relationships[i][j],
                            relationships[i][k] + relationships[k][j],
                        );
                    }
                }
            }
        }
    }

    let mut min_index = 0;
    let mut min_sum_kevin_bacon = std::usize::MAX;

    for i in 1..=n {
        let mut sum_kevin_bacon = 0;

        for j in 1..=n {
            sum_kevin_bacon += relationships[i][j];
        }

        if sum_kevin_bacon < min_sum_kevin_bacon {
            min_sum_kevin_bacon = sum_kevin_bacon;
            min_index = i;
        }
    }

    writeln!(out, "{}", min_index).unwrap();
}
