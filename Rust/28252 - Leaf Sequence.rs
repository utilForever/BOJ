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

    // Process impossible cases
    if nums[n - 1] > 2 || (n >= 2 && nums[n - 1] == 1 && nums[n - 2] == 1) {
        writeln!(out, "-1").unwrap();
        return;
    }

    for i in 0..n - 1 {
        if nums[i + 1] > nums[i] {
            writeln!(out, "-1").unwrap();
            return;
        }
    }

    writeln!(out, "{}", nums.iter().sum::<i64>()).unwrap();

    let mut offset = nums[n - 1];

    if offset > 1 {
        writeln!(out, "1 2").unwrap();
    }

    for i in (0..n - 1).rev() {
        for j in 1..=nums[i] {
            writeln!(
                out,
                "{} {}",
                (offset - nums[i + 1] + j).min(offset),
                j + offset
            )
            .unwrap();
        }

        offset += nums[i];
    }
}
