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
    let mut limits = vec![0; n];

    for i in 0..n {
        limits[i] = scan.token::<usize>();
    }

    let m = scan.token::<usize>();
    let mut weights = vec![0; m];

    for i in 0..m {
        weights[i] = scan.token::<usize>();
    }

    limits.sort_by(|a, b| b.cmp(a));
    weights.sort_by(|a, b| b.cmp(a));

    if weights[0] > limits[0] {
        writeln!(out, "-1").unwrap();
        return;
    }

    let mut ret = 0;

    while !weights.is_empty() {
        for i in 0..limits.len() {
            for j in 0..weights.len() {
                if limits[i] >= weights[j] {
                    weights.remove(j);
                    break;
                }
            }
        }

        ret += 1;
    }

    writeln!(out, "{}", ret).unwrap();
}
