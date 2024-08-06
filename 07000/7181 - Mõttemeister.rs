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

    let password = scan.token::<String>();
    let password = password.chars().collect::<Vec<_>>();
    let n = scan.token::<i64>();

    for _ in 0..n {
        let candidate = scan.token::<String>();
        let candidate = candidate.chars().collect::<Vec<_>>();
        let mut visited = vec![false; candidate.len()];
        let mut ret_a = 0;
        let mut ret_b = 0;

        for c1 in password.iter() {
            for (i, c2) in candidate.iter().enumerate() {
                if !visited[i] && c1 == c2 {
                    visited[i] = true;
                    ret_a += 1;
                    break;
                }
            }
        }

        for (c1, c2) in password.iter().zip(candidate.iter()) {
            if c1 == c2 {
                ret_b += 1;
            }
        }

        writeln!(out, "{ret_a} {ret_b}").unwrap();
    }
}
