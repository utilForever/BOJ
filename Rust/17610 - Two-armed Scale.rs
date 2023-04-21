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

fn check_weights(weights: &Vec<i64>, ret: &mut Vec<bool>, idx: usize, sum: i64) {
    if idx == weights.len() {
        if sum < 0 {
            return;
        }
        
        ret[sum as usize] = true;
        return;
    }

    check_weights(weights, ret, idx + 1, sum);
    check_weights(weights, ret, idx + 1, sum + weights[idx]);
    check_weights(weights, ret, idx + 1, sum - weights[idx]);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let k = scan.token::<usize>();
    let mut weights = vec![0; k];

    for i in 0..k {
        weights[i] = scan.token::<i64>();
    }

    let sum = weights.iter().sum::<i64>();
    let mut ret = vec![false; sum as usize + 1];

    check_weights(&weights, &mut ret, 0, 0);

    writeln!(out, "{}", ret.iter().filter(|&&x| !x).count()).unwrap();
}
