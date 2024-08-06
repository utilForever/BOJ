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

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut blubs = vec![0; n];

    for i in 0..n {
        blubs[i] = scan.token::<i64>();
    }

    for _ in 0..m {
        let (a, b, c) = (
            scan.token::<i64>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        match a {
            1 => {
                blubs[b - 1] = c as i64;
            }
            2 => {
                for i in b - 1..c {
                    blubs[i] ^= 1;
                }
            }
            3 => {
                for i in b - 1..c {
                    blubs[i] = 0;
                }
            }
            4 => {
                for i in b - 1..c {
                    blubs[i] = 1;
                }
            }
            _ => unreachable!(),
        }
    }

    for blub in blubs {
        write!(out, "{blub} ").unwrap();
    }

    writeln!(out).unwrap();
}
