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

    let (k, n) = (scan.token::<usize>(), scan.token::<usize>());
    let mut nums_raw = vec![0; k];
    let mut max = 0;

    for i in 0..k {
        nums_raw[i] = scan.token::<u128>();
        max = max.max(nums_raw[i]);
    }

    let mut nums = vec![String::new(); k];

    for i in 0..k {
        nums[i] = nums_raw[i].to_string();
    }

    for _ in 0..n - k {
        nums.push(max.to_string());
    }

    for i in 0..n - 1 {
        let mut idx = i;

        for j in i + 1..n {
            let mut str1 = nums[j].clone();
            str1.push_str(&nums[idx]);

            let mut str2 = nums[idx].clone();
            str2.push_str(&nums[j]);

            if str1.parse::<u128>().unwrap() > str2.parse::<u128>().unwrap() {
                idx = j;
            }
        }

        nums.swap(i, idx);
    }

    let mut ret = String::new();

    for num in nums.iter() {
        ret.push_str(&num);
    }

    writeln!(out, "{ret}").unwrap();
}
