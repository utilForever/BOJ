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

    let (n, h, w) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut s = vec![vec![' '; n * w]; h];

    for i in 0..h {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            s[i][j] = c;
        }
    }

    for i in (0..n * w).step_by(w) {
        let mut c = None;

        for j in i..i + w {
            for k in 0..h {
                if s[k][j] != '?' {
                    c = Some(s[k][j]);
                    break;
                }
            }
        }

        write!(
            out,
            "{}",
            match c {
                Some(c) => c,
                None => '?',
            }
        )
        .unwrap();
    }

    writeln!(out).unwrap();
}
