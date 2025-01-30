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

    let (h, w, q) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut paper = vec![vec![0; w + 1]; h + 1];
    let mut masking = vec![vec![false; w + 1]; h + 1];

    for _ in 0..q {
        let command = scan.token::<i64>();

        if command == 1 {
            let (x, y, c) = (
                scan.token::<usize>(),
                scan.token::<usize>(),
                scan.token::<i64>(),
            );

            for i in x..=x + 1 {
                for j in y..=y + 1 {
                    if !masking[i][j] {
                        paper[i][j] = c;
                    }
                }
            }
        } else {
            let (x, y) = (scan.token::<usize>(), scan.token::<usize>());

            for i in x..=x + 1 {
                for j in y..=y + 1 {
                    masking[i][j] = true;
                }
            }
        }
    }

    for i in 1..=h {
        for j in 1..=w {
            write!(out, "{} ", paper[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
