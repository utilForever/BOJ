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

fn check(mut val: usize) -> bool {
    let mut nums = Vec::new();

    while val > 0 {
        nums.push(val % 10);
        val /= 10;
    }

    nums.reverse();

    let mut idx_left = 0;
    let mut idx_right = nums.len() - 1;

    while nums[idx_left] < nums[idx_left + 1] {
        idx_left += 1;

        if idx_left == nums.len() - 1 {
            break;
        }
    }

    while nums[idx_right] < nums[idx_right - 1] {
        idx_right -= 1;

        if idx_right == 0 {
            break;
        }
    }

    idx_left != nums.len() - 1 && idx_right != 0 && idx_left == idx_right
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut prefix_sum = vec![0; 1_000_001];

    for i in 100..=1_000_000 {
        prefix_sum[i] = prefix_sum[i - 1] + if check(i) { 1 } else { 0 };
    }

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());
        writeln!(out, "{}", prefix_sum[b] - prefix_sum[a - 1]).unwrap();
    }
}
