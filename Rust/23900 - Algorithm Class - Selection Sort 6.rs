use io::Write;
use std::{collections::HashMap, io, str};

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
    let mut a = vec![0; n];
    let mut b = vec![0; n];
    let mut sorted = vec![0; n];

    for i in 0..n {
        a[i] = scan.token::<i64>();
        sorted[i] = a[i];
    }

    for i in 0..n {
        b[i] = scan.token::<i64>();
    }

    if a == b {
        writeln!(out, "1").unwrap();
        return;
    }

    sorted.sort();

    let mut idxes = HashMap::new();

    for (idx, &val) in a.iter().enumerate() {
        idxes.insert(val, idx);
    }

    for idx in (0..n).rev() {
        if a[idx] != sorted[idx] {
            let ret = vec![a[idx], sorted[idx]];
            a.swap(idx, *idxes.get(&sorted[idx]).unwrap());

            let val1 = *idxes.get(&ret[0]).unwrap();
            let val2 = *idxes.get(&ret[1]).unwrap();
            idxes.insert(ret[0], val2);
            idxes.insert(ret[1], val1);

            if a == b {
                writeln!(out, "1").unwrap();
                return;
            }
        }
    }

    writeln!(out, "0").unwrap();
}
