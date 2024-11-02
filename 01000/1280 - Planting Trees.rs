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

const MOD: i64 = 1_000_000_007;

fn update(bit: &mut Vec<i64>, index: usize, value: i64) {
    let mut idx = index + 1;

    while idx < bit.len() {
        bit[idx] = (bit[idx] + value) % MOD;
        idx += idx & (!idx + 1);
    }
}

fn query(bit: &Vec<i64>, index: usize) -> i64 {
    let mut idx = (index + 1).min(bit.len() - 1);
    let mut ret = 0;

    while idx > 0 {
        ret = (ret + bit[idx]) % MOD;
        idx -= idx & (!idx + 1);
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut trees = vec![0; n];

    for i in 0..n {
        trees[i] = scan.token::<i64>();
    }

    let mut bit_cnt = vec![0; 200002];
    let mut bit_sum = vec![0; 200002];
    let mut total_cnt = 0;
    let mut total_sum = 0;
    let mut ret = 1;

    for i in 1..n {
        update(&mut bit_cnt, trees[i - 1] as usize, 1);
        update(&mut bit_sum, trees[i - 1] as usize, trees[i - 1]);

        total_cnt += 1;
        total_sum += trees[i - 1];

        let cnt_left = query(&bit_cnt, trees[i] as usize - 1);
        let sum_left = query(&bit_sum, trees[i] as usize - 1);

        let cnt_right = total_cnt - cnt_left;
        let sum_right = total_sum - sum_left;
        let cost = ((trees[i] * cnt_left - sum_left) % MOD
            + (sum_right - trees[i] * cnt_right) % MOD
            + MOD)
            % MOD;

        ret = (ret * cost) % MOD;
    }

    writeln!(out, "{ret}").unwrap();
}
