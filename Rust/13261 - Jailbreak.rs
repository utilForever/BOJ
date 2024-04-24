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

#[inline]
fn calculate(powers_acc: &Vec<i64>, i: usize, j: usize) -> i64 {
    if i > j {
        0
    } else {
        (powers_acc[j] - powers_acc[i - 1]) * (j - i + 1) as i64
    }
}

fn calculate_min(
    risks: &mut Vec<Vec<i64>>,
    idxes: &mut Vec<Vec<i64>>,
    powers_acc: &Vec<i64>,
    idx: usize,
    left: i64,
    right: i64,
    p_left: usize,
    p_right: usize,
) {
    if left > right {
        return;
    }

    let mid = ((left + right) / 2) as usize;
    risks[idx][mid] = -1;
    idxes[idx][mid] = -1;

    for i in p_left..=p_right {
        let risk = risks[idx - 1][i] + calculate(powers_acc, i + 1, mid);

        if risks[idx][mid] == -1 || risks[idx][mid] > risk {
            risks[idx][mid] = risk;
            idxes[idx][mid] = i as i64;
        }
    }

    calculate_min(
        risks,
        idxes,
        powers_acc,
        idx,
        left,
        mid as i64 - 1,
        p_left,
        idxes[idx][mid] as usize,
    );
    calculate_min(
        risks,
        idxes,
        powers_acc,
        idx,
        mid as i64 + 1,
        right,
        idxes[idx][mid] as usize,
        p_right,
    );
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (l, g) = (scan.token::<usize>(), scan.token::<usize>());
    let mut powers = vec![0; l + 1];

    for i in 1..=l {
        powers[i] = scan.token::<i64>();
    }

    let powers_acc = powers
        .iter()
        .scan(0, |acc, &x| {
            *acc = *acc + x;
            Some(*acc)
        })
        .collect::<Vec<_>>();

    let mut risks = vec![vec![0; l + 1]; g + 1];
    let mut idxes = vec![vec![0; l + 1]; g + 1];

    for i in 1..=l {
        risks[1][i] = calculate(&powers_acc, 1, i);
        idxes[1][i] = 0;
    }

    for i in 2..=g {
        calculate_min(&mut risks, &mut idxes, &powers_acc, i, 0, l as i64, 0, l);
    }

    writeln!(out, "{}", risks[g][l]).unwrap();
}
