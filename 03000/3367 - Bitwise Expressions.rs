use io::Write;
use std::{collections::BinaryHeap, io, str};

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

fn interval(a: i64, b: i64) -> (i64, i64) {
    if a == b {
        return (a, 0);
    }

    let p = 63u32 - (a ^ b).leading_zeros();
    let mask = (1i64 << (p + 1)) - 1;

    (a & !mask, b & mask)
}

fn prefix_or_max(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        return a | b;
    }

    let bit = 1i64 << (63u32 - a.max(b).leading_zeros());

    if (a & bit) != 0 && (b & bit) != 0 {
        (bit << 1) - 1
    } else if (a & bit) != 0 {
        bit | prefix_or_max(a ^ bit, b)
    } else {
        bit | prefix_or_max(a, b ^ bit)
    }
}

fn merge(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    (a.0 | b.0, prefix_or_max(a.1, b.1))
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (_, p) = (scan.token::<usize>(), scan.token::<usize>());
    let mut variables = vec![0; p];

    for i in 0..p {
        variables[i] = scan.token::<i64>();
    }

    let mut groups = Vec::with_capacity(p);

    for &var in variables.iter() {
        let mut heap = BinaryHeap::new();

        for _ in 0..var {
            let (a, b) = interval(scan.token::<i64>(), scan.token::<i64>());
            heap.push((a, b));
        }

        while heap.len() > 1 {
            let a = heap.pop().unwrap();
            let b = heap.pop().unwrap();
            heap.push(merge(a, b));
        }

        groups.push(heap.pop().unwrap());
    }

    let mut ret = 0;

    for bit in (0..=30).rev() {
        let val = ret | (1i64 << bit);

        if groups.iter().all(|&(a, b)| (val & !a) <= b) {
            ret = val;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
