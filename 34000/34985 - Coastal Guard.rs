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

    let (w, h) = (scan.token::<f64>(), scan.token::<f64>());
    let n = scan.token::<usize>();
    let mut guards = vec![0; n + 1];

    for i in 1..=n {
        guards[i] = scan.token::<i64>();
    }

    let base = w * h * 0.5;
    let mut prefix_sum = vec![0.0; n + 1];

    for i in 2..=n {
        let diff = (guards[i] - guards[i - 1]) as f64;
        let area = base * diff / (w + diff);

        prefix_sum[i] = prefix_sum[i - 1] + area;
    }

    let q = scan.token::<i64>();

    for _ in 0..q {
        let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
        writeln!(out, "{:.9}", base + prefix_sum[r] - prefix_sum[l]).unwrap();
    }
}
