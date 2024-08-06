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

    let (r, c) = (scan.token::<usize>(), scan.token::<usize>());
    let mut parking_lot = vec![vec![' '; c]; r];

    for i in 0..r {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            parking_lot[i][j] = c;
        }
    }

    let mut ret = vec![0; 5];

    for i in 0..r - 1 {
        for j in 0..c - 1 {
            let capacity = vec![
                parking_lot[i][j],
                parking_lot[i + 1][j],
                parking_lot[i][j + 1],
                parking_lot[i + 1][j + 1],
            ];

            if capacity.contains(&'#') {
                continue;
            }

            let cnt = capacity.iter().filter(|&&c| c == 'X').count();
            ret[cnt] += 1;
        }
    }

    for val in ret {
        writeln!(out, "{val}").unwrap();
    }
}
