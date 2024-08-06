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
    let mut from = vec![false; n + 1];
    let mut to = vec![false; n + 1];

    for _ in 0..m {
        let (a, b) = (scan.token::<usize>(), scan.token::<usize>());

        from[a] = true;
        to[b] = true;
    }

    let s = scan.token::<usize>();

    if from[s] {
        writeln!(out, "NOJAM").unwrap();
    } else if to[s] {
        if n - m == 1 {
            writeln!(out, "NOJAM").unwrap();
        } else if n - m == 2 {
            for i in 1..=n {
                if i == s {
                    continue;
                }

                if !from[i] && !to[i] {
                    writeln!(out, "NOJAM").unwrap();
                    break;
                }
            }
        } else {
            writeln!(out, "{}", n - m).unwrap();
        }
    } else {
        if n - m == 2 {
            writeln!(out, "NOJAM").unwrap();
        } else {
            writeln!(out, "{}", n - m - 1).unwrap();
        }
    }
}
