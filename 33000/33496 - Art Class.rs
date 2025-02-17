use io::Write;
use std::{
    collections::{BTreeSet, HashSet},
    io, str,
};

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

    let n = scan.token::<usize>();
    let mut x = vec![0; n];
    let mut y = vec![0; n];

    for i in 0..n {
        x[i] = scan.token::<i64>();
        y[i] = scan.token::<i64>();
    }

    let mut left = BTreeSet::new();
    let mut right = BTreeSet::new();
    let mut axis = HashSet::new();

    for i in 0..n {
        left.insert(x[i] - y[i]);
        right.insert(x[i] + y[i]);
        axis.insert(x[i] - y[i]);
        axis.insert(x[i] + y[i]);
    }

    let left = left.into_iter().collect::<Vec<_>>();
    let right = right.into_iter().collect::<Vec<_>>();

    let mut r = 0;
    let mut ret = axis.len();

    for &l in left.iter() {
        while r < right.len() && right[r] <= l {
            r += 1;
        }

        ret += right.len() - r;
    }

    writeln!(out, "{ret}").unwrap();
}
