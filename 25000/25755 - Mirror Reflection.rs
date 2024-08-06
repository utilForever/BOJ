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

    let (w, n) = (scan.token::<char>(), scan.token::<usize>());
    let mut arr = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            arr[i][j] = scan.token::<i64>();
        }
    }

    let mut ret = vec![vec![0; n]; n];

    match w {
        'L' | 'R' => {
            for i in 0..n {
                for j in 0..n {
                    ret[i][j] = arr[i][n - j - 1];
                }
            }
        }
        'U' | 'D' => {
            for i in 0..n {
                for j in 0..n {
                    ret[i][j] = arr[n - i - 1][j];
                }
            }
        }
        _ => {}
    }

    for i in 0..n {
        for j in 0..n {
            write!(
                out,
                "{} ",
                match ret[i][j] {
                    1 => "1",
                    2 => "5",
                    3 => "?",
                    4 => "?",
                    5 => "2",
                    6 => "?",
                    7 => "?",
                    8 => "8",
                    9 => "?",
                    _ => unreachable!(),
                }
            )
            .unwrap();
        }

        writeln!(out).unwrap();
    }
}
