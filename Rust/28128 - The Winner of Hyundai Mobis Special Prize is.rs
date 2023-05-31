use io::Write;
use std::{io, str, collections::BTreeSet};

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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut names = vec![vec![String::new(); m]; n];

    for i in 0..n {
        for j in 0..m {
            names[i][j] = scan.token::<String>();
        }
    }

    let mut ret = BTreeSet::new();

    for i in 0..n {
        for j in 0..m {
            if i < n.saturating_sub(1) && names[i][j] == names[i + 1][j] {
                ret.insert(names[i][j].clone());
            }

            if i < n.saturating_sub(2) && names[i][j] == names[i + 2][j] {
                ret.insert(names[i][j].clone());
            }

            if j < m.saturating_sub(1) && names[i][j] == names[i][j + 1] {
                ret.insert(names[i][j].clone());
            }

            if j < m.saturating_sub(2) && names[i][j] == names[i][j + 2] {
                ret.insert(names[i][j].clone());
            }
        }
    }

    if ret.is_empty() {
        writeln!(out, "MANIPULATED").unwrap();
    } else {
        for name in ret.iter() {
            writeln!(out, "{name}").unwrap();
        }
    }
}
