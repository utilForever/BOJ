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

static INF: i64 = 1e9 as i64;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i64>();
    let mut cost = vec![vec![INF; 52]; 52];

    for _ in 0..n {
        let (a, _, b) = (
            scan.token::<String>(),
            scan.token::<String>(),
            scan.token::<String>(),
        );
        let (a, b) = (a.chars().next().unwrap(), b.chars().next().unwrap());

        let idx_a = if a.is_lowercase() {
            a as usize - 'a' as usize + 26
        } else {
            a as usize - 'A' as usize
        };
        let idx_b = if b.is_lowercase() {
            b as usize - 'a' as usize + 26
        } else {
            b as usize - 'A' as usize
        };

        cost[idx_a][idx_b] = 1;
    }

    for k in 0..52 {
        for i in 0..52 {
            for j in 0..52 {
                if i == j {
                    continue;
                }

                cost[i][j] = std::cmp::min(cost[i][j], cost[i][k] + cost[k][j]);
            }
        }
    }

    let mut cnt = 0;

    for i in 0..52 {
        for j in 0..52 {
            if i == j {
                continue;
            }

            if cost[i][j] == INF {
                continue;
            }

            cnt += 1;
        }
    }

    writeln!(out, "{cnt}").unwrap();

    for i in 0..52 {
        for j in 0..52 {
            if i == j {
                continue;
            }

            if cost[i][j] == INF {
                continue;
            }

            let a = if i < 26 {
                (i + 'A' as usize) as u8 as char
            } else {
                (i + 'a' as usize - 26) as u8 as char
            };

            let b = if j < 26 {
                (j + 'A' as usize) as u8 as char
            } else {
                (j + 'a' as usize - 26) as u8 as char
            };

            writeln!(out, "{a} => {b}").unwrap();
        }
    }
}
