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
    let mut b = vec![0; n];

    for i in 0..n {
        b[i] = scan.token::<i64>();
    }

    if b[n - 1] != n as i64 + 1 {
        writeln!(out, "No").unwrap();
        return;
    }

    for i in 0..n {
        if b[i] > i as i64 + 2 {
            writeln!(out, "No").unwrap();
            return;
        }

        if i < n - 1 && b[i] > b[i + 1] {
            writeln!(out, "No").unwrap();
            return;
        }
    }

    let mut cnt = vec![0; n + 2];
    let mut skipped = Vec::new();

    for i in 0..n {
        cnt[b[i] as usize] += 1;
    }

    for i in 1..=n {
        if cnt[i] == 0 {
            skipped.push(i as i64);
        }
    }

    let mut a = vec![0; n];
    let mut idx = 1;

    a[0] = skipped[0];

    for i in 1..n {
        if b[i] == b[i - 1] {
            a[i] = skipped[idx];
            idx += 1;
        } else {
            a[i] = b[i - 1];
        }
    }

    writeln!(out, "Yes").unwrap();

    for val in a {
        write!(out, "{val} ").unwrap();
    }

    writeln!(out).unwrap();
}
