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

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut nums = vec![0; n + 1];

    for i in 1..=n {
        nums[i] = scan.token::<i64>();
    }

    let mut diff = vec![0; n + 1];

    for i in 1..n {
        diff[i] = nums[i + 1] - nums[i];
    }

    let mut cnt_minus = diff.iter().filter(|&&x| x < 0).count() as i64;

    for _ in 0..q {
        let cmd = scan.token::<i64>();

        if cmd == 1 {
            let (i, j, v) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i64>(),
            );

            if i > 1 {
                let prev_minus = if diff[i - 1] < 0 { 1 } else { 0 };
                diff[i - 1] += v;
                let curr_minus = if diff[i - 1] < 0 { 1 } else { 0 };

                cnt_minus += curr_minus - prev_minus;
            }

            if j < n {
                let prev_minus = if diff[j] < 0 { 1 } else { 0 };
                diff[j] -= v;
                let curr_minus = if diff[j] < 0 { 1 } else { 0 };

                cnt_minus += curr_minus - prev_minus;
            }
        } else {
            writeln!(out, "{}", if cnt_minus == 0 { "YES" } else { "NO" }).unwrap();
        }
    }
}
