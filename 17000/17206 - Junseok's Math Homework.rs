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

    let mut nums = vec![0; 80_001];

    for i in (3..=80_000).step_by(3) {
        nums[i] += i;
    }

    for i in (7..=80_000).step_by(7) {
        nums[i] += i;
    }

    for i in (21..=80_000).step_by(21) {
        nums[i] -= i;
    }

    let mut prefix_sum = vec![0; 80_001];

    for i in 1..=80_000 {
        prefix_sum[i] = prefix_sum[i - 1] + nums[i];
    }

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        writeln!(out, "{}", prefix_sum[n]).unwrap();
    }
}
