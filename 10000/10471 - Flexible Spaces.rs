use io::Write;
use std::{collections::BTreeSet, io, str};

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

    let (w, p) = (scan.token::<i64>(), scan.token::<usize>());
    let mut partitions = vec![0; p];

    let mut ret = BTreeSet::new();
    ret.insert(w);

    for i in 0..p {
        partitions[i] = scan.token::<i64>();
        ret.insert(partitions[i]);
        ret.insert(w - partitions[i]);
    }

    for i in 0..p - 1 {
        for j in i + 1..p {
            ret.insert(partitions[j] - partitions[i]);
        }
    }

    for val in ret {
        write!(out, "{} ", val).unwrap();
    }

    writeln!(out).unwrap();
}
