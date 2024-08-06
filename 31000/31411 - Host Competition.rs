use io::Write;
use std::{collections::HashSet, io, str};

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

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut difficulties = Vec::with_capacity(n * k);

    for i in 1..=n {
        for _ in 1..=k {
            let difficulty = scan.token::<i64>();
            difficulties.push((difficulty, i));
        }
    }

    difficulties.sort();

    let mut algorithms_kind = HashSet::new();
    let mut algorithms_cnt = vec![0; n + 1];
    let mut left = 0;
    let mut right = 0;
    let mut ret = i64::MAX;

    while right < n * k {
        while algorithms_kind.len() < n {
            algorithms_kind.insert(difficulties[right].1);
            algorithms_cnt[difficulties[right].1] += 1;
            right += 1;

            if right == n * k {
                break;
            }
        }

        while algorithms_kind.len() == n {
            algorithms_cnt[difficulties[left].1] -= 1;

            if algorithms_cnt[difficulties[left].1] == 0 {
                algorithms_kind.remove(&difficulties[left].1);
            }

            left += 1;
        }

        ret = ret.min(difficulties[right - 1].0 - difficulties[left - 1].0);
    }

    writeln!(out, "{ret}").unwrap();
}
