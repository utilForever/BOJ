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

    let (a, b) = (scan.token::<String>(), scan.token::<String>());
    let a = a.chars().collect::<Vec<_>>();
    let b = b.chars().collect::<Vec<_>>();
    let mut is_found = false;
    let mut idx_a = 0;
    let mut idx_b = 0;

    for i in 0..a.len() {
        for j in 0..b.len() {
            if a[i] == b[j] {
                is_found = true;
                idx_a = i;
                idx_b = j;
                break;
            }
        }

        if is_found {
            break;
        }
    }

    let mut ret = vec![vec!['.'; a.len()]; b.len()];

    for i in 0..a.len() {
        ret[idx_b][i] = a[i];
    }

    for i in 0..b.len() {
        ret[i][idx_a] = b[i];
    }

    for i in 0..b.len() {
        for j in 0..a.len() {
            write!(out, "{}", ret[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
