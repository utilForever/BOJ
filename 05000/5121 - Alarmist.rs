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

    let k = scan.token::<i64>();

    for i in 1..=k {
        let (n, w) = (scan.token::<usize>(), scan.token::<usize>());
        let mut samples = vec![0; n];

        for j in 0..n {
            samples[j] = scan.token::<i64>();
        }

        let mut ret = Vec::with_capacity(n - w + 1);

        samples.windows(w).for_each(|window| {
            let sum: i64 = window.iter().sum();
            let avg = sum as f64 / w as f64;
            ret.push(avg.floor() as i64);
        });

        let max = *ret.iter().max().unwrap();
        let min = *ret.iter().min().unwrap();

        writeln!(out, "Data Set {i}:").unwrap();
        writeln!(out, "{}", max - min).unwrap();

        if i != k {
            writeln!(out).unwrap();
        }
    }
}
