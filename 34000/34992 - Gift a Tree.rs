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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let k = scan.token::<usize>();
    let mut s = vec![0; k];

    for i in 0..k {
        s[i] = scan.token::<usize>();
    }

    if s[0] != 0 {
        writeln!(out, "-1").unwrap();
        return;
    }

    let n = s[k - 1] / 2;
    let mut endpoints = Vec::with_capacity(k - 1);

    for val in s {
        let idx = val / 2;

        if idx != 0 {
            endpoints.push(idx);
        }
    }

    let mut perm = Vec::with_capacity(n);
    let mut prev = 0;

    for endpoint in endpoints {
        for i in (prev..endpoint).rev() {
            perm.push(i);
        }

        prev = endpoint;
    }

    let mut child_left = vec![0; n];
    let mut child_right = vec![0; n];

    for i in 1..=n {
        let idx = perm[i - 1];

        child_left[idx] = if i == 1 { 0 } else { 2 * perm[i - 2] + 1 };
        child_right[idx] = 2 * i;
    }

    writeln!(out, "{n}").unwrap();

    for i in 0..n {
        writeln!(out, "{} {}", child_left[i], child_right[i]).unwrap();
    }
}
