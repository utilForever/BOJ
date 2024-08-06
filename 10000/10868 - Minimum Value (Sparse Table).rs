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
    let mut arr = vec![vec![i64::MAX; 17]; 100001];

    for i in 1..=n {
        arr[i][0] = scan.token::<i64>();
    }

    for j in 1..=16 {
        for i in 1..=n {
            if i + (1 << (j - 1)) > n {
                continue;
            }

            arr[i][j] = arr[i][j - 1].min(arr[i + (1 << (j - 1))][j - 1]);
        }
    }

    for _ in 0..m {
        let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
        let idx = ((r - l + 1) as f64).log2() as usize;

        writeln!(out, "{}", arr[l][idx].min(arr[r - (1 << idx) + 1][idx])).unwrap();
    }
}
