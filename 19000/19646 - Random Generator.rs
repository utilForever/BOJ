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

struct FenwickTree {
    n: usize,
    data: Vec<i64>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        FenwickTree {
            n,
            data: vec![0; n + 1],
        }
    }

    fn update(&mut self, mut idx: usize, delta: i64) {
        while idx <= self.n {
            self.data[idx] += delta;
            idx += idx & (!idx + 1);
        }
    }

    fn query(&self, mut k: i64) -> usize {
        let mut ret = 0;
        let msb = usize::BITS - 1 - self.n.leading_zeros();
        let mut mask = 1usize << msb;

        while mask > 0 {
            let next = ret + mask;

            if next <= self.n && self.data[next] < k {
                ret = next;
                k -= self.data[next];
            }

            mask >>= 1;
        }

        ret + 1
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut w = vec![0; n + 1];
    let mut p = vec![0; n + 1];

    for i in 1..=n {
        w[i] = scan.token::<i64>();
    }

    for i in 1..=n {
        p[i] = scan.token::<i64>();
    }

    let mut fenwick = FenwickTree::new(n);

    for i in 1..=n {
        fenwick.update(i, w[i]);
    }

    let mut ret = vec![0; n + 1];

    for i in 1..=n {
        let idx = fenwick.query(p[i]);

        ret[i] = idx;
        fenwick.update(idx, -w[idx]);
    }

    for i in 1..=n {
        write!(out, "{} ", ret[i]).unwrap();
    }

    writeln!(out).unwrap();
}
