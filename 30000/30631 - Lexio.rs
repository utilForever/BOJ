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
    let mut cards = vec![0; n + 1];

    for i in 1..=n {
        cards[i] = scan.token::<i64>();
    }

    let sum = cards.iter().sum::<i64>();
    let mut scores = vec![0; n + 1];

    for i in 1..=n {
        scores[i] = sum - cards[i] * n as i64;
    }

    let mut prefix_sum_scores = vec![0; n + 1];
    let mut prefix_sum_scores_sorted = vec![(0, 0); n + 1];
    let mut prefix_sum_zero = vec![0; n + 1];

    for i in 1..=n {
        prefix_sum_scores[i] = prefix_sum_scores[i - 1] + scores[i];
    }

    for i in 1..=n {
        prefix_sum_scores_sorted[i] = (prefix_sum_scores[i], i);
    }

    prefix_sum_scores_sorted.sort();

    for i in 1..=n {
        prefix_sum_zero[i] = prefix_sum_zero[i - 1] + if prefix_sum_scores[i] == 0 { 1 } else { 0 };
    }

    let mut prev_val = i64::MIN;
    let mut prev_idx = 0;
    let mut change = 0;
    let mut change_max = 0;

    for (val, idx) in prefix_sum_scores_sorted {
        if val == 0 {
            continue;
        }

        if val == prev_val {
            change = (change - (prefix_sum_zero[idx] - prefix_sum_zero[prev_idx])).max(0);
            change += 1;

            change_max = change_max.max(change as i64);
        } else {
            change = 1;
        }

        prev_val = val;
        prev_idx = idx;
    }

    change_max = prefix_sum_zero[n] + (change_max - 1).max(0);

    writeln!(out, "{}", n as i64 - change_max).unwrap();
}
