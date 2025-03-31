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

    let (n, m, k, l) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut celestials = vec![(0, 0); k];

    for i in 0..k {
        celestials[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let dist = n - 1 + m - 1;

    if l - dist < 0 || (l - dist) % 2 == 1 {
        writeln!(out, "-1").unwrap();
        return;
    }

    let dist_y = n - 1;
    let dist_x = m - 1;

    for _ in (0..l - dist_y - dist_x).step_by(2) {
        write!(out, "DU").unwrap();
    }

    for _ in 0..dist_y {
        write!(out, "D").unwrap();
    }

    for _ in 0..dist_x {
        write!(out, "R").unwrap();
    }

    writeln!(out).unwrap();
}
