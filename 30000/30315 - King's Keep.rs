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

    let k = scan.token::<usize>();
    let mut keeps = vec![(0, 0); k];

    for i in 0..k {
        keeps[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let mut ret = f64::MAX;

    for i in 0..k {
        let mut sum = 0.0;

        for j in 0..k {
            if i == j {
                continue;
            }

            let dist = (keeps[i].0 - keeps[j].0).pow(2) + (keeps[i].1 - keeps[j].1).pow(2);
            sum += (dist as f64).sqrt();
        }

        ret = ret.min(sum);
    }

    writeln!(out, "{:.9}", ret / (k - 1) as f64).unwrap();
}
