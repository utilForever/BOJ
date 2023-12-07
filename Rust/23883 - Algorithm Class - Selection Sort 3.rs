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

    let (n, k) = (scan.token::<usize>(), scan.token::<i64>());
    let mut nums = vec![0; n];
    let mut sorted = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
        sorted[i] = nums[i];
    }

    sorted.sort();

    let mut idxes = HashMap::new();

    for (idx, &val) in nums.iter().enumerate() {
        idxes.insert(val, idx);
    }

    let mut cnt = 0;

    for idx in (0..n).rev() {
        if nums[idx] != sorted[idx] {
            let ret = vec![nums[idx], sorted[idx]];
            nums.swap(idx, *idxes.get(&sorted[idx]).unwrap());

            let val1 = *idxes.get(&ret[0]).unwrap();
            let val2 = *idxes.get(&ret[1]).unwrap();
            idxes.insert(ret[0], val2);
            idxes.insert(ret[1], val1);

            cnt += 1;

            if cnt == k {
                writeln!(out, "{} {}", ret[0], ret[1]).unwrap();
                return;
            }
        }
    }

    writeln!(out, "-1").unwrap();
}
