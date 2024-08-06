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
    let s = scan.token::<String>();
    let s = s.chars().collect::<Vec<_>>();

    let mut idx1 = n - 1;

    while s[idx1] == 'A' || s[idx1] == 'E' || s[idx1] == 'I' || s[idx1] == 'O' || s[idx1] == 'U' {
        idx1 -= 1;

        if idx1 + 1 < m {
            writeln!(out, "NO").unwrap();
            return;
        }
    }

    let mut idx2 = idx1 - 1;

    while s[idx2] != 'A' {
        idx2 -= 1;

        if idx2 + 2 < m {
            writeln!(out, "NO").unwrap();
            return;
        }
    }

    let mut idx3 = idx2 - 1;

    while s[idx3] != 'A' {
        idx3 -= 1;

        if idx3 + 3 < m {
            writeln!(out, "NO").unwrap();
            return;
        }
    }

    let mut ret = s.iter().take(m - 3).collect::<String>();
    ret.push('A');
    ret.push('A');
    ret.push(s[idx1]);

    writeln!(out, "YES").unwrap();
    writeln!(out, "{ret}").unwrap();
}
