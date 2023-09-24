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

    let mut pop_count_a = vec![0; 500001];
    let mut pop_count_b = vec![0; 500001];
    let mut cnt_ones = 0;

    for i in 1..=500000 {
        cnt_ones = cnt_ones.max((i as i64).count_ones());

        let val = ((1 << cnt_ones) - 1) as i64;

        pop_count_a[i] = val.count_ones() as i64;
        pop_count_b[i] = (i as i64 - val).count_ones() as i64;
    }

    for i in 1..=500000 {
        pop_count_a[i] += pop_count_a[i - 1];
        pop_count_b[i] += pop_count_b[i - 1];
    }

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, mut a, mut b) = (
            scan.token::<usize>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        if a < b {
            std::mem::swap(&mut a, &mut b);
        }

        writeln!(out, "{}", a * pop_count_a[n] + b * pop_count_b[n]).unwrap();
    }
}
