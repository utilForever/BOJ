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

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let positions = scan.token::<String>();
    let mut positions = positions.chars().collect::<Vec<_>>();
    let mut ret = 0;

    for i in 0..n {
        if positions[i] == 'H' || positions[i] == ' ' {
            continue;
        }

        let left = (i as i32 - k as i32).max(0) as usize;
        let right = (i + k).min(n - 1);

        for j in left..=right {
            if positions[j] == 'H' {
                positions[j] = ' ';
                ret += 1;
                break;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
