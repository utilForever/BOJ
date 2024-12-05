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

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    nums.sort();

    let mut ret = vec![(i64::MAX, i64::MIN); 300001];

    for i in 1..=300000 {
        for val in (0..=300000 + i).step_by(i as usize) {
            let pos = nums.partition_point(|&x| x < val);

            if pos > 0 {
                ret[i as usize].1 = ret[i as usize].1.max(nums[pos - 1] % i);
            }

            if pos < n {
                ret[i as usize].0 = ret[i as usize].0.min(nums[pos] % i);
            }
        }
    }

    for _ in 0..q {
        let m = scan.token::<usize>();
        writeln!(out, "{} {}", ret[m].0, ret[m].1).unwrap();
    }
}
