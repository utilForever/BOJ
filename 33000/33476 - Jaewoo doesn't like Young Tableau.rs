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
            buf_str: Vec::new(),
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

fn calculate_val(v1: &Vec<i64>, v2: &Vec<i64>, x: i64, h: usize, c: i64, d: i64) -> i64 {
    let mut start = 0;
    let mut end = h;

    while start < end {
        let mid = (start + end + 1) / 2;

        if v1[mid] < x {
            end = mid - 1;
        } else {
            start = mid;
        }
    }

    c * (v2[start] - x * (start as i64 + 1))
        + d * (x * (h as i64 - start as i64) - v2[h] + v2[start])
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, h) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let (c, d) = (scan.token::<i64>(), scan.token::<i64>());
    let mut ret = 0;
    let mut ret_list = vec![(0, 0); n];

    for i in 0..n {
        let mut initial = vec![0; h];
        let mut prefix_sum = vec![0; h];

        for j in 0..h {
            initial[j] = scan.token::<i64>();
            prefix_sum[j] = initial[j];

            if j > 0 {
                prefix_sum[j] += prefix_sum[j - 1];
            }
        }

        let mut start = initial[h - 1];
        let mut end = initial[0];

        while end - start >= 3 {
            let mid1 = start + (end - start) / 3;
            let mid2 = end - (end - start) / 3;

            let val1 = calculate_val(&initial, &prefix_sum, mid1, h - 1, c, d);
            let val2 = calculate_val(&initial, &prefix_sum, mid2, h - 1, c, d);

            if val1 > val2 {
                start = mid1;
            } else {
                end = mid2;
            }
        }

        let mut temp = (
            calculate_val(&initial, &prefix_sum, start, h - 1, c, d),
            start,
        );

        for j in start + 1..=end {
            temp = temp.min((calculate_val(&initial, &prefix_sum, j, h - 1, c, d), j));
        }

        ret += temp.0;
        ret_list[i] = (temp.1, i + 1);
    }

    for _ in 0..m {
        let (_, _) = (scan.token::<i64>(), scan.token::<i64>());
    }

    writeln!(out, "{ret}").unwrap();

    for i in 0..n {
        writeln!(out, "{} {}", ret_list[i].0, ret_list[i].1).unwrap();
    }
}
