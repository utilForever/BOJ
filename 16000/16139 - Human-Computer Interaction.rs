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

    let s = scan.token::<String>().chars().collect::<Vec<_>>();
    let mut prefix_sum = vec![vec![0; 26]; s.len() + 1];

    for i in 0..s.len() {
        for j in 0..26 {
            prefix_sum[i + 1][j] = prefix_sum[i][j] + (s[i] as u8 - b'a' == j as u8) as i64;
        }
    }

    let q = scan.token::<i64>();

    for _ in 0..q {
        let (a, l, r) = (
            scan.token::<char>(),
            scan.token::<usize>() + 1,
            scan.token::<usize>() + 1,
        );
        let a = (a as u8 - b'a') as usize;

        writeln!(out, "{}", prefix_sum[r][a] - prefix_sum[l - 1][a]).unwrap();
    }
}
