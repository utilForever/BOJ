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

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut backpack_first = vec![0; n + 1];
    let mut backpack_second = vec![0; n + 1];

    for i in 1..=n {
        backpack_first[i] = scan.token::<i64>();
    }

    for i in 1..=n {
        backpack_second[i] = scan.token::<i64>();
    }

    let sum_first = backpack_first.iter().sum::<i64>();
    let sum_second = backpack_second.iter().sum::<i64>();

    let mut prefix_sum_top_first = vec![0; n + 1];
    let mut prefix_sum_top_second = vec![0; n + 1];

    for i in 1..=n {
        prefix_sum_top_first[i] = prefix_sum_top_first[i - 1] + backpack_first[n - i + 1];
        prefix_sum_top_second[i] = prefix_sum_top_second[i - 1] + backpack_second[n - i + 1];
    }

    let mut prefix_sum_bottom_first = vec![0; n + 1];
    let mut prefix_sum_bottom_second = vec![0; n + 1];

    for i in 1..=n {
        prefix_sum_bottom_first[i] = prefix_sum_bottom_first[i - 1] + backpack_first[i];
        prefix_sum_bottom_second[i] = prefix_sum_bottom_second[i - 1] + backpack_second[i];
    }

    let mut remain_weight_first = vec![i64::MAX; k + 1];
    let mut remain_weight_second = vec![i64::MAX; k + 1];

    for idx in 0..=n.min(k) {
        let mut weight_min = i64::MAX;

        for idx_top in 0..=n.min(idx) {
            let idx_bottom = idx - idx_top;
            let weight_removed = prefix_sum_top_first[idx_top] + prefix_sum_bottom_first[idx_bottom];
            let weight_remain = sum_first - weight_removed;

            weight_min = weight_min.min(weight_remain);
        }

        remain_weight_first[idx] = weight_min;
    }

    for idx in 0..=n.min(k) {
        let mut weight_min = i64::MAX;

        for idx_top in 0..=n.min(idx) {
            let idx_bottom = idx - idx_top;
            let weight_removed = prefix_sum_top_second[idx_top] + prefix_sum_bottom_second[idx_bottom];
            let weight_remain = sum_second - weight_removed;

            weight_min = weight_min.min(weight_remain);
        }

        remain_weight_second[idx] = weight_min;
    }

    let mut ret = i64::MAX;

    for idx1 in 0..=k.min(n) {
        let idx2 = (k - idx1).min(n);

        let weight_first = remain_weight_first[idx1];
        let weight_second = remain_weight_second[idx2];
        let weight_max = weight_first.max(weight_second);

        ret = ret.min(weight_max);
    }

    writeln!(out, "{ret}").unwrap();
}
