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

    let (n, s, r) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut kayaks = vec!['N'; n];

    for _ in 0..s {
        kayaks[scan.token::<usize>() - 1] = 'D';
    }

    for _ in 0..r {
        let pos = scan.token::<usize>() - 1;
        kayaks[pos] = if kayaks[pos] == 'D' { 'N' } else { 'R' };
    }

    for i in 0..n {
        if kayaks[i] == 'N' || kayaks[i] == 'R' {
            continue;
        }

        let left = (i as i32 - 1).max(0) as usize;
        let right = (i + 1).min(n - 1);

        for j in left..=right {
            if kayaks[j] == 'R' {
                kayaks[i] = 'N';
                kayaks[j] = 'N';
                break;
            }
        }
    }

    writeln!(out, "{}", kayaks.iter().filter(|&x| *x == 'D').count()).unwrap();
}
