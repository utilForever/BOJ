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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<i64>(), scan.token::<usize>());
    let mut nums = vec![0; m];

    for i in 0..m {
        nums[i] = scan.token::<i64>();
    }

    nums.insert(0, 1);

    let mut ret = 0;

    for i in 0..m {
        let left = if nums[i] <= nums[i + 1] {
            nums[i + 1] - nums[i]
        } else {
            n - nums[i] + nums[i + 1]
        };
        let right = if nums[i] >= nums[i + 1] {
            nums[i] - nums[i + 1]
        } else {
            nums[i] + n - nums[i + 1]
        };

        ret += left.min(right);
    }

    writeln!(out, "{ret}").unwrap();
}
