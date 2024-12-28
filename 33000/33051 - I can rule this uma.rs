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

fn backtrack(
    start: usize,
    remain: usize,
    base: &[usize],
    selected: &mut Vec<usize>,
    ret: &mut Vec<Vec<usize>>,
) {
    if remain == 0 {
        ret.push(selected.clone());
        return;
    }

    for idx in start..base.len() {
        selected.push(base[idx]);
        backtrack(idx + 1, remain - 1, base, selected, ret);
        selected.pop();
    }
}

fn selected_three(r: usize) -> Vec<Vec<usize>> {
    let base = vec![0, 1, 2];
    let mut selected = Vec::new();
    let mut ret = Vec::new();

    backtrack(0, r, &base, &mut selected, &mut ret);

    ret
}

#[derive(Clone, Copy, Default)]
struct Diff {
    sum: i64,
    one: i64,
    two: i64,
    three: i64,
    four: i64,
}

impl Diff {
    fn new(sum: i64, one: i64, two: i64, three: i64, four: i64) -> Self {
        Self {
            sum,
            one,
            two,
            three,
            four,
        }
    }
}

fn check(val: i64, greater: bool, strict: bool) -> bool {
    if greater {
        if strict {
            val > 0
        } else {
            val >= 0
        }
    } else {
        if strict {
            val < 0
        } else {
            val <= 0
        }
    }
}

fn satisfy(
    d1: i64,
    d2: i64,
    d3: i64,
    d4: i64,
    up: &[(usize, bool)],
    down: &[(usize, bool)],
    diffs: &[Diff],
) -> bool {
    for &(idx, strict) in up {
        let diff = &diffs[idx];
        let val = diff.sum + diff.one * d1 + diff.two * d2 + diff.three * d3 + diff.four * d4;

        if !check(val, true, strict) {
            return false;
        }
    }

    for &(idx, strict) in down {
        let diff = &diffs[idx];
        let val = diff.sum + diff.one * d1 + diff.two * d2 + diff.three * d3 + diff.four * d4;

        if !check(val, false, strict) {
            return false;
        }
    }

    true
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k, m) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut sum_base = vec![0; 5];
    let mut rank_cnt = vec![vec![0; 5]; 5];

    for _ in 0..n {
        let (a1, a2, a3, a4, s1, s2, s3, s4) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        sum_base[a1] += s1;
        rank_cnt[a1][1] += 1;

        sum_base[a2] += s2;
        rank_cnt[a2][2] += 1;

        sum_base[a3] += s3;
        rank_cnt[a3][3] += 1;

        sum_base[a4] += s4;
        rank_cnt[a4][4] += 1;
    }

    let players: Vec<usize> = (1..=4).filter(|&x| x != k).collect();
    let selected = selected_three(m - 1);
    let mut diff = vec![Diff::default(); 5];

    for i in 1..=4 {
        diff[i] = Diff::new(
            sum_base[i] - sum_base[k],
            rank_cnt[i][1] - rank_cnt[k][1],
            rank_cnt[i][2] - rank_cnt[k][2],
            rank_cnt[i][3] - rank_cnt[k][3],
            rank_cnt[i][4] - rank_cnt[k][4],
        );
    }

    for select in selected.iter() {
        let mut up = Vec::new();
        let mut down = Vec::new();

        for (idx, &player) in players.iter().enumerate() {
            if select.contains(&idx) {
                if player < k {
                    up.push((player, false));
                } else {
                    up.push((player, true));
                }
            } else {
                if player < k {
                    down.push((player, true));
                } else {
                    down.push((player, false));
                }
            }
        }

        for d4 in -100..=100 {
            for d3 in d4..=100 {
                for d2 in d3..=100 {
                    let d1 = -(d2 + d3 + d4);

                    if d1 < d2 || d1 > 100 || d1 < -100 {
                        continue;
                    }

                    if satisfy(d1, d2, d3, d4, &up, &down, &diff) {
                        writeln!(out, "{d1} {d2} {d3} {d4}").unwrap();
                        return;
                    }
                }
            }
        }
    }

    writeln!(out, "-1").unwrap();
}
