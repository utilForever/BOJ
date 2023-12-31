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

#[derive(Default, Clone)]
struct Nation {
    num: i64,
    rank: i64,
    medal_gold: i64,
    medal_silver: i64,
    medal_bronze: i64,
}

impl Nation {
    fn new(num: i64, medal_gold: i64, medal_silver: i64, medal_bronze: i64) -> Self {
        Self {
            num,
            rank: 0,
            medal_gold,
            medal_silver,
            medal_bronze,
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut nations = vec![Nation::default(); n];

    for i in 0..n {
        let (num, gold, silver, bronze) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        nations[i] = Nation::new(num, gold, silver, bronze);
    }

    nations.sort_by(|a, b| a.num.cmp(&b.num));

    // Calculate rank
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            if nations[i].medal_gold < nations[j].medal_gold {
                nations[i].rank += 1;
            } else if nations[i].medal_gold == nations[j].medal_gold {
                if nations[i].medal_silver < nations[j].medal_silver {
                    nations[i].rank += 1;
                } else if nations[i].medal_silver == nations[j].medal_silver {
                    if nations[i].medal_bronze < nations[j].medal_bronze {
                        nations[i].rank += 1;
                    }
                }
            }
        }
    }

    writeln!(out, "{}", nations[k - 1].rank + 1).unwrap();
}
