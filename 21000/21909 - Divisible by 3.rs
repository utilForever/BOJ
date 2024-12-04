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

    let n = scan.token::<usize>();
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    let deltas = vec![(0, 0), (1, 1), (2, 1)];
    let mut counts = HashMap::new();
    counts.insert((0, 0), 1_u64);

    let mut sum = 0;
    let mut sum_squared = 0;
    let mut ret: u64 = 0;

    for &num in nums.iter() {
        let num_squared = (num * num) % 3;

        sum = (sum + num) % 3;
        sum_squared = (sum_squared + num_squared) % 3;

        for &(delta_sum, delta_sum_squared) in deltas.iter() {
            let required_s = (sum + 3 - delta_sum) % 3;
            let required_ss = (sum_squared + 3 - delta_sum_squared) % 3;

            if let Some(&count) = counts.get(&(required_s, required_ss)) {
                ret += count;
            }
        }

        *counts.entry((sum, sum_squared)).or_insert(0) += 1;
    }

    writeln!(out, "{ret}").unwrap();
}
