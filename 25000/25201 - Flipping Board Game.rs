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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut orig_row = vec![0; 100001];
    let mut orig_col = vec![0; 100001];
    let mut dest_row = vec![0; 100001];
    let mut dest_col = vec![0; 100001];

    for _ in 0..n {
        let (x, y) = (scan.token::<usize>(), scan.token::<usize>());
        orig_col[x] += 1;
        orig_row[y] += 1;
    }

    for _ in 0..m {
        let (x, y) = (scan.token::<usize>(), scan.token::<usize>());
        dest_col[x] += 1;
        dest_row[y] += 1;
    }

    for i in 1..=100000 {
        orig_col[i] %= 2;
        orig_row[i] %= 2;
        dest_col[i] %= 2;
        dest_row[i] %= 2;

        if orig_col[i] != dest_col[i] || orig_row[i] != dest_row[i] {
            writeln!(out, "NO").unwrap();
            return;
        }
    }

    writeln!(out, "YES").unwrap();
}
