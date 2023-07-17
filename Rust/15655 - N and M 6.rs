use io::Write;
use std::io::{BufWriter, StdoutLock};
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

fn check(
    out: &mut BufWriter<StdoutLock>,
    nums: &mut Vec<i64>,
    ret: &mut [i64; 9],
    num: usize,
    n: usize,
    m: usize,
) {
    if num == m + 1 {
        for i in 1..=m {
            write!(out, "{} ", ret[i]).unwrap();
        }

        writeln!(out).unwrap();
    } else {
        for i in 1..=n {
            if *ret.iter().max().unwrap() < nums[i - 1] {
                ret[num] = nums[i - 1];
                check(out, nums, ret, num + 1, n, m);
                ret[num] = 0;
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    nums.sort();

    let mut ret = [0; 9];

    check(&mut out, &mut nums, &mut ret, 1, n, m);
}
