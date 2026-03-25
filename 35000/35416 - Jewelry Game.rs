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

fn best_two_max(next: &Vec<i64>, row: &Vec<i64>) -> (i64, usize, i64) {
    let mut val_best1 = i64::MIN;
    let mut val_best2 = i64::MIN;
    let mut idx_best1 = 0;

    for i in 0..next.len() {
        let val = next[i] + row[i];

        if val > val_best1 {
            val_best2 = val_best1;
            val_best1 = val;
            idx_best1 = i;
        } else if val > val_best2 {
            val_best2 = val;
        }
    }

    (val_best1, idx_best1, val_best2)
}

fn best_two_min(next: &Vec<i64>, row: &Vec<i64>) -> (i64, usize, i64) {
    let mut val_best1 = i64::MAX;
    let mut val_best2 = i64::MAX;
    let mut idx_best1 = 0;

    for i in 0..next.len() {
        let val = next[i] - row[i];

        if val < val_best1 {
            val_best2 = val_best1;
            val_best1 = val;
            idx_best1 = i;
        } else if val < val_best2 {
            val_best2 = val;
        }
    }

    (val_best1, idx_best1, val_best2)
}

fn calculate_level_max(ret: &mut Vec<i64>, next: &Vec<i64>, row: &Vec<i64>) {
    let n = next.len();
    let (val_best1, idx_best1, val_best2) = best_two_max(next, row);

    for i in 0..n {
        let val = if n == 1 {
            i64::MIN
        } else if i == idx_best1 {
            val_best2
        } else {
            val_best1
        };

        ret[i] = next[i].max(val);
    }
}

fn calculate_level_min(ret: &mut Vec<i64>, next: &Vec<i64>, row: &Vec<i64>) {
    let n = next.len();
    let (val_best1, idx_best1, val_best2) = best_two_min(next, row);

    for i in 0..n {
        let val = if n == 1 {
            i64::MAX
        } else if i == idx_best1 {
            val_best2
        } else {
            val_best1
        };

        ret[i] = next[i].min(val);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (r, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut values = vec![vec![0; k]; r];

    for i in 0..r {
        for j in 0..k {
            values[i][j] = scan.token::<i64>();
        }
    }

    let mut dp_curr = vec![0; k];
    let mut dp_next = vec![0; k];

    for i in (2..=r).rev() {
        if i % 2 == 1 {
            calculate_level_max(&mut dp_curr, &dp_next, &values[i - 1]);
        } else {
            calculate_level_min(&mut dp_curr, &dp_next, &values[i - 1]);
        }

        std::mem::swap(&mut dp_curr, &mut dp_next);
    }

    let mut ret = i64::MIN;

    for i in 0..k {
        ret = ret.max(values[0][i] + dp_next[i]);
    }

    if ret > 0 {
        writeln!(out, "djangg7 {ret}").unwrap();
    } else {
        writeln!(out, "ibasic {}", -ret).unwrap();
    }
}
