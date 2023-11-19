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
    let mut a = vec![0; n];
    let mut b = vec![0; n];
    let mut x = vec![0; n];

    for i in 0..n {
        a[i] = scan.token::<i64>();
    }

    for i in 0..n {
        b[i] = scan.token::<i64>();
    }

    for i in 0..n {
        x[i] = scan.token::<i64>();
    }

    let mut ret = vec![0; n];

    for i in 0..n {
        let val = (a[i] - b[i]).abs();

        if val % x[i] != 0 {
            writeln!(out, "-1").unwrap();
            return;
        }

        ret[i] = val / x[i];
    }

    if ret.iter().all(|&x| x % 2 == 0) || ret.iter().all(|&x| x % 2 == 1) {
        writeln!(out, "{}", ret.iter().max().unwrap()).unwrap();
    } else {
        writeln!(out, "-1").unwrap();
    }
}
