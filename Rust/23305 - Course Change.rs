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
    let mut arr_a = vec![0; n];
    let mut arr_b = vec![0; n];

    for i in 0..n {
        arr_a[i] = scan.token::<usize>();
    }

    for i in 0..n {
        arr_b[i] = scan.token::<usize>();
    }

    arr_a.sort();
    arr_b.sort();

    let mut idx_a = 0;
    let mut idx_b = 0;
    let mut ret = 0;

    while idx_a < n && idx_b < n {
        if arr_a[idx_a] < arr_b[idx_b] {
            idx_a += 1;
        } else if arr_a[idx_a] > arr_b[idx_b] {
            idx_b += 1;
        } else {
            ret += 1;
            idx_a += 1;
            idx_b += 1;
        }
    }

    writeln!(out, "{}", n - ret).unwrap();
}
