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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    let mut prefix_sum = vec![0; n + 1];

    for i in 0..n {
        prefix_sum[i + 1] = prefix_sum[i] + nums[i];
    }

    let mut ret = 0;

    for i in 0..n {
        // Clockwise
        let dir_cw = (i + 1) % n;

        if dir_cw <= k {
            let sum_cw = if dir_cw == 0 { 0 } else { prefix_sum[i + 1] };
            let val = sum_cw + (k - dir_cw) as i64 * nums[i];
            ret = ret.max(val);
        }

        // Counter-clockwise
        let dir_ccw = n - 1 - i;

        if dir_ccw <= k {
            let sum_ccw = if dir_ccw == 0 {
                0
            } else {
                prefix_sum[n] - prefix_sum[i + 1]
            };
            let val = sum_ccw + (k - dir_ccw) as i64 * nums[i];
            ret = ret.max(val);
        }
    }

    writeln!(out, "{ret}").unwrap();
}
