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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    let n = scan.token::<usize>();
    let mut votes = vec![0; n];

    for i in 0..n {
        votes[i] = scan.token::<i64>();
    }

    let mut votes_sorted = votes.clone();
    votes_sorted.sort_unstable();

    let mut votes_unique = votes_sorted.clone();
    votes_unique.dedup();

    let mut sum = votes.iter().sum::<i64>();
    let mut steal_min_to_win = Vec::with_capacity(votes_unique.len());
    let mut idx_vote = 0;
    let mut idx_target = 0;

    for &vote in votes_unique.iter() {
        while idx_vote < n && votes_sorted[idx_vote] <= vote {
            idx_vote += 1;
        }

        while idx_target < idx_vote {
            sum -= votes_sorted[idx_target];
            idx_target += 1;
        }

        loop {
            let cnt = n - idx_target;
            let numerator = vote + sum;
            let denominator = cnt as i64 + 1;
            let target = (numerator + denominator - 1) / denominator;
            let next = if idx_target < n {
                votes_sorted[idx_target]
            } else {
                target
            };

            if target <= next {
                steal_min_to_win.push(target - vote);
                break;
            } else {
                if idx_target < n {
                    sum -= votes_sorted[idx_target];
                    idx_target += 1;
                } else {
                    steal_min_to_win.push(target - vote);
                    break;
                }
            }
        }
    }

    for vote in votes.iter() {
        let idx = votes_unique.binary_search(&vote).unwrap();
        write!(out, "{} ", steal_min_to_win[idx]).unwrap();
    }

    writeln!(out).unwrap();
}
