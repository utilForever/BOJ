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
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    nums.sort();

    let mut num_cnt = [0; 8001];
    for num in nums.iter() {
        num_cnt[*num as usize + 4000] += 1;
    }

    let mut candidates = Vec::new();
    let mut candidate = 0;

    for i in 0..=8000 {
        if num_cnt[i] > candidate {
            candidates.clear();
            candidates.push(i as i64 - 4000);

            candidate = num_cnt[i];
        } else if num_cnt[i] == candidate {
            candidates.push(i as i64 - 4000);
        }
    }

    let arithmetic_mean = (nums.iter().sum::<i64>() as f64 / n as f64).round() as i64;
    let median = nums[n / 2];
    let most_frequent_value = if candidates.len() == 1 {
        candidates[0]
    } else {
        candidates[1]
    };
    let range = nums.iter().max().unwrap() - nums.iter().min().unwrap();

    writeln!(out, "{}", arithmetic_mean).unwrap();
    writeln!(out, "{}", median).unwrap();
    writeln!(out, "{}", most_frequent_value).unwrap();
    writeln!(out, "{}", range).unwrap();
}
