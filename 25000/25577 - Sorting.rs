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
    let mut nums = vec![(0, 0); n];
    let mut positions = vec![0; n];

    for i in 0..n {
        nums[i] = (scan.token::<i64>(), i);
        positions[nums[i].1] = i;
    }

    let mut nums_sorted = nums.clone();
    nums_sorted.sort();

    let mut ret = 0;

    for i in 0..nums.len() {
        if nums[i] != nums_sorted[i] {
            let pos = positions[nums_sorted[i].1];
            nums.swap(i, pos);
            ret += 1;

            positions[nums[pos].1] = pos;
        }
    }

    writeln!(out, "{ret}").unwrap();
}