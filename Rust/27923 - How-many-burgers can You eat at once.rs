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

    let (n, k, l) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut hamburgers = vec![0; n];
    let mut cokes = vec![0; n];

    for i in 0..n {
        hamburgers[i] = scan.token::<i64>();
    }

    for _ in 0..k {
        let t = scan.token::<usize>() - 1;

        cokes[t] += 1;

        if t + l < n {
            cokes[t + l] -= 1;
        }
    }

    for i in 1..n {
        cokes[i] += cokes[i - 1];
    }

    hamburgers.sort_by(|a, b| b.cmp(a));
    cokes.sort();

    let mut ret = 0;

    for hamburger in hamburgers {
        if let Some(coke) = cokes.pop() {
            if coke <= 30 {
                ret += hamburger >> coke;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
