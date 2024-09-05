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

    let n = scan.token::<usize>();
    let mut faucets = vec![0; n];
    let mut flag = vec![true; n];

    for i in 0..n {
        faucets[i] = scan.token::<i64>();
    }

    let mut sum = faucets.iter().sum::<i64>();

    writeln!(out, "{sum}").unwrap();

    let q = scan.token::<i64>();

    for _ in 0..q {
        let command = scan.token::<i64>();

        if command == 1 {
            let (i, x) = (scan.token::<usize>(), scan.token::<i64>());

            if flag[i - 1] {
                sum -= faucets[i - 1];
            }

            faucets[i - 1] = x;

            if flag[i - 1] {
                sum += faucets[i - 1];
            }
        } else {
            let i = scan.token::<usize>();

            if flag[i - 1] {
                sum -= faucets[i - 1];
                flag[i - 1] = false;
            } else {
                sum += faucets[i - 1];
                flag[i - 1] = true;
            }
        }

        writeln!(out, "{sum}").unwrap();
    }
}
