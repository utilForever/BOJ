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
    let mut wheel = vec!['?'; n];
    let mut idx = 0;

    for _ in 0..k {
        let (mut s, letter) = (scan.token::<usize>(), scan.token::<char>());
        s %= n;

        idx = if idx < s {
            idx + n - s
        } else {
            idx - s
        };

        if wheel[idx] != '?' && wheel[idx] != letter {
            writeln!(out, "!").unwrap();
            return;
        } else {
            wheel[idx] = letter;
        }
    }

    for i in 0..n {
        for j in i + 1..n {
            if wheel[i] != '?' && wheel[i] == wheel[j] {
                writeln!(out, "!").unwrap();
                return;
            }
        }
    }

    for i in 0..n {
        write!(out, "{}", wheel[(i + idx) % n]).unwrap();
    }

    writeln!(out).unwrap();
}
