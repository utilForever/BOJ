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
    let mut diff = vec![0; n];
    let mut dist_fall = 0;

    for i in 0..n {
        let (a, b) = (scan.token::<i64>(), scan.token::<i64>());

        diff[i] = a - b;
        dist_fall += b;
    }

    diff.sort_by(|a, b| b.cmp(a));

    let ground = diff.iter().sum::<i64>().min(0);
    let diff_sum = diff
        .iter()
        .enumerate()
        .map(|(idx, val)| val * (n - idx) as i64)
        .sum::<i64>();

    writeln!(out, "{}", diff_sum + dist_fall - (n as i64 * ground)).unwrap();
}
