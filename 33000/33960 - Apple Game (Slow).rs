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
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<usize>();
    }

    let mut score_max_deleted = vec![vec![i64::MIN; n]; n];
    let mut score_max_chain = vec![vec![vec![i64::MIN; 11]; n]; n];

    for i in 0..n {
        score_max_chain[i][i][nums[i]] = 0;
    }

    for i in 2..=n {
        for l in 0..=n - i {
            let r = l + i - 1;

            for j in 0..=10 {
                score_max_chain[l][r][j] = i64::MIN;
            }

            for x in l + 1..=r {
                let diff = if x == l + 1 {
                    0
                } else {
                    if score_max_deleted[l + 1][x - 1] == i64::MIN {
                        continue;
                    }

                    score_max_deleted[l + 1][x - 1]
                };

                for t in 1..=10 {
                    let val = score_max_chain[x][r][t];

                    if val == i64::MIN {
                        continue;
                    }

                    let sum = nums[l] + t;

                    if sum <= 10 {
                        score_max_chain[l][r][sum] = score_max_chain[l][r][sum].max(diff + val);
                    }
                }
            }

            let mut val_max = i64::MIN;

            for x in l..r {
                let left = score_max_deleted[l][x];

                if left == i64::MIN {
                    continue;
                }

                let right = score_max_deleted[x + 1][r];

                if right == i64::MIN {
                    continue;
                }

                val_max = val_max.max(left + right);
            }

            let chain = score_max_chain[l][r][10];

            if chain != i64::MIN {
                let cand = chain + 1;
                val_max = val_max.max(cand);
            }

            score_max_deleted[l][r] = val_max;
        }
    }

    let mut ret = vec![0; n + 1];

    for i in 0..n {
        ret[i + 1] = ret[i + 1].max(ret[i]);

        for j in i..n {
            let val = score_max_deleted[i][j];

            if val == i64::MIN {
                continue;
            }

            ret[j + 1] = ret[j + 1].max(ret[i] + val);
        }
    }

    writeln!(out, "{}", ret[n]).unwrap();
}
