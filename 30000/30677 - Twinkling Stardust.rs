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

    let (n, k, c, r) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut base = vec![0; k];
    let mut s = vec![0; k];
    let mut p = vec![0; k];
    let mut skill = vec![0; k];

    for i in 0..k {
        base[i] = scan.token::<i64>();
    }

    for i in 0..k {
        s[i] = scan.token::<i64>();
    }

    for i in 0..k {
        p[i] = scan.token::<i64>();
    }

    let mut combo = 0;
    let mut fatigue = 0_i64;
    let mut stardust = 0;

    for _ in 0..n {
        let l = scan.token::<usize>();

        if l == 0 {
            combo = 0;
            fatigue = (fatigue - r).max(0);
        } else {
            stardust += base[l - 1] * (100 + combo * c) * (100 + skill[l - 1] * s[l - 1]) / 10000;
            fatigue += p[l - 1];

            if fatigue > 100 {
                writeln!(out, "-1").unwrap();
                return;
            }

            skill[l - 1] += 1;
            combo += 1;
        }
    }

    writeln!(out, "{stardust}").unwrap();
}
