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

    let pos_yuumi = (scan.token::<i64>(), scan.token::<i64>());
    let mut positions = [(0, 0); 3];

    for i in 0..3 {
        positions[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let mut ret = f64::MAX;

    // Cases:
    // Yuumi -> A -> B -> C
    // Yuumi -> A -> C -> B
    // Yuumi -> B -> A -> C
    // Yuumi -> B -> C -> A
    // Yuumi -> C -> A -> B
    // Yuumi -> C -> B -> A
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                if i == j || j == k || i == k {
                    continue;
                }

                let mut dist = 0.0;
                dist += (((pos_yuumi.0 - positions[i].0).pow(2)
                    + (pos_yuumi.1 - positions[i].1).pow(2)) as f64)
                    .sqrt();
                dist += (((positions[i].0 - positions[j].0).pow(2)
                    + (positions[i].1 - positions[j].1).pow(2)) as f64)
                    .sqrt();
                dist += (((positions[j].0 - positions[k].0).pow(2)
                    + (positions[j].1 - positions[k].1).pow(2)) as f64)
                    .sqrt();

                ret = ret.min(dist);
            }
        }
    }

    writeln!(out, "{}", ret as i64).unwrap();
}
