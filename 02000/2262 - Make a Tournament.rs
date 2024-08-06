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
    let mut ranks = vec![0; n];

    for i in 0..n {
        ranks[i] = scan.token::<i64>();
    }

    let mut ret = 0;

    for _ in 0..n - 1 {
        let mut rank_max = 0;
        let mut idx = 0;

        for i in 0..ranks.len() {
            if ranks[i] > rank_max {
                rank_max = ranks[i];
                idx = i;
            }
        }

        if idx == 0 {
            ret += (ranks[0] - ranks[1]).abs();

            if ranks[0] > ranks[1] {
                ranks.remove(0);
            } else {
                ranks.remove(1);
            }
        } else if idx == ranks.len() - 1 {
            ret += (ranks[idx] - ranks[idx - 1]).abs();

            if ranks[idx] > ranks[idx - 1] {
                ranks.remove(idx);
            } else {
                ranks.remove(idx - 1);
            }
        } else {
            let diff1 = (ranks[idx] - ranks[idx - 1]).abs();
            let diff2 = (ranks[idx] - ranks[idx + 1]).abs();

            if diff1 > diff2 {
                ret += diff2;

                if ranks[idx] > ranks[idx + 1] {
                    ranks.remove(idx);
                } else {
                    ranks.remove(idx + 1);
                }
            } else if diff1 < diff2 {
                ret += diff1;

                if ranks[idx] > ranks[idx - 1] {
                    ranks.remove(idx);
                } else {
                    ranks.remove(idx - 1);
                }
            } else {
                ret += diff1;

                if ranks[idx + 1] > ranks[idx - 1] {
                    ranks.remove(idx + 1);
                } else {
                    ranks.remove(idx - 1);
                }
            }
        }
    }

    writeln!(out, "{}", ret).unwrap();
}
