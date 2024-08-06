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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
        let mut a = vec![0; n];
        let mut b = vec![0; m];

        for i in 0..n {
            a[i] = scan.token::<i64>();
        }

        for i in 0..m {
            b[i] = scan.token::<i64>();
        }

        a.sort();
        b.sort();

        let mut idx_a = a.len() as i64 - 1;
        let mut idx_b = b.len() as i64 - 1;
        let mut ret = 0;

        while idx_a >= 0 && idx_b >= 0 {
            if a[idx_a as usize] > b[idx_b as usize] {
                ret += idx_b + 1;
                idx_a -= 1;
            } else {
                idx_b -= 1;
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
