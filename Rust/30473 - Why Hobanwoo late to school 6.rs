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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut nums = vec![0; n + 1];
    let mut xor_prefix = vec![0; n + 2];
    let mut xor_suffix = vec![0; n + 2];

    for i in 1..=n {
        nums[i] = scan.token::<i64>();
    }

    for i in 1..=n {
        xor_prefix[i] = xor_prefix[i - 1] ^ nums[i];
    }

    for i in (1..=n).rev() {
        xor_suffix[i] = xor_suffix[i + 1] ^ nums[i];
    }

    let mut ret = 0;

    for i in 1..=(n - m + 1) {
        let mut value_left = 0;
        let mut value_middle = 0;
        let mut value_right = 0;

        // Calculate left side from middle to start
        let mut idx = i as i64;
        let mut pos = 0;
        let mut a = 0;
        let mut b = 0;

        while idx > 0 {
            if pos % 2 == 0 {
                a ^= nums[idx as usize];
                b ^= nums[idx as usize + 1];
                value_left = value_left.max((a ^ xor_prefix[idx as usize - 1]) + b);
            } else {
                a ^= nums[idx as usize + 1];
                b ^= nums[idx as usize];
                value_left = value_left.max(a + (b ^ xor_prefix[idx as usize - 1]));
            }

            idx -= 2;
            pos += 1;
        }

        // Calculate middle side
        let mut pos = i + 2;

        while pos < i + m - 2 {
            value_middle += nums[pos];
            pos += 1;
        }

        // Calculate right side from middle to end
        let mut idx = (i + m - 1) as i64;
        let mut pos = 0;
        let mut a = 0;
        let mut b = 0;

        while idx <= n as i64 {
            if pos % 2 == 0 {
                a ^= nums[idx as usize];
                b ^= nums[idx as usize - 1];
                value_right = value_right.max((a ^ xor_suffix[idx as usize + 1]) + b);
            } else {
                a ^= nums[idx as usize - 1];
                b ^= nums[idx as usize];
                value_right = value_right.max(a + (b ^ xor_suffix[idx as usize + 1]));
            }

            idx += 2;
            pos += 1;
        }

        ret = ret.max(value_left + value_middle + value_right);
    }

    writeln!(out, "{ret}").unwrap();
}
