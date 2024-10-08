use io::Write;
use std::{collections::HashMap, io, str};

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

    let (n, m, q) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut reviewers = vec![Vec::new(); m + 1];
    let mut is_heavy = vec![false; m + 1];
    let mut reviewers_heavy = vec![Vec::new(); m + 1];
    let mut contests_heavy = vec![Vec::new(); n + 1];
    let mut pain_sum = vec![0; m + 1];
    let mut pain_direct = vec![0; n + 1];
    let mut pain_last = vec![HashMap::new(); n + 1];

    for _ in 0..q {
        let cmd = scan.token::<i64>();

        if cmd == 1 {
            let (j, i) = (scan.token::<usize>(), scan.token::<usize>());

            if is_heavy[j] {
                let pos_reviewrs = reviewers_heavy[j].iter().position(|&x| x == i);

                if let Some(idx) = pos_reviewrs {
                    reviewers_heavy[j].swap_remove(idx);

                    let accumulated_pain = pain_sum[j] - pain_last[i].remove(&j).unwrap();
                    pain_direct[i] += accumulated_pain;

                    let pos_contests = contests_heavy[i].iter().position(|&x| x == j);

                    if let Some(idx) = pos_contests {
                        contests_heavy[i].swap_remove(idx);
                    }
                } else {
                    reviewers_heavy[j].push(i);
                    contests_heavy[i].push(j);
                    pain_last[i].insert(j, pain_sum[j]);
                }
            } else {
                let pos = reviewers[j].iter().position(|&x| x == i);

                if let Some(idx) = pos {
                    reviewers[j].swap_remove(idx);
                } else {
                    reviewers[j].push(i);

                    if reviewers[j].len() > 300 {
                        is_heavy[j] = true;
                        reviewers_heavy[j] = reviewers[j].clone();

                        for &reviewer in reviewers_heavy[j].iter() {
                            contests_heavy[reviewer].push(j);
                            pain_last[reviewer].insert(j, 0);
                        }

                        reviewers[j].clear();
                    }
                }
            }
        } else if cmd == 2 {
            let (j, x) = (scan.token::<usize>(), scan.token::<i64>());

            if is_heavy[j] {
                pain_sum[j] += x;
            } else {
                for &reviewer in reviewers[j].iter() {
                    pain_direct[reviewer] += x;
                }
            }
        } else {
            let i = scan.token::<usize>();
            let mut ret = pain_direct[i];

            for &idx in contests_heavy[i].iter() {
                let pain_accumulated = pain_sum[idx] - pain_last[i][&idx];
                ret += pain_accumulated;
            }

            writeln!(out, "{ret}").unwrap();
        }
    }
}
