use io::Write;
use std::{cmp, io, str};

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
    let mut robot = vec![0; n];

    let (r, m) = (scan.token::<i64>(), scan.token::<i64>());

    for i in 0..n {
        robot[i] = scan.token::<i64>();
    }

    robot.sort();

    for i in 0..n {
        robot.push(robot[i] + m);
    }

    let mut sum = 0;
    let mut max_sum = 0;

    for i in 1..robot.len() {
        let dist = robot[i] - robot[i - 1] - 2 * r;
        sum += dist;

        max_sum = cmp::max(max_sum, sum);

        if sum < 0 {
            sum = 0;
        }
    }

    writeln!(out, "{}", (max_sum + 1) / 2).unwrap();
}
