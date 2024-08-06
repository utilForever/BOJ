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

    let (n, d) = (scan.token::<usize>(), scan.token::<i64>());
    let mut grid = vec![vec![' '; n]; n];

    for i in 0..n {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            grid[i][j] = c;
        }
    }

    for i in 0..n {
        for j in 0..n {
            write!(
                out,
                "{}",
                match grid[i][j] {
                    'd' => {
                        if d == 1 {
                            'q'
                        } else {
                            'b'
                        }
                    }
                    'b' => {
                        if d == 1 {
                            'p'
                        } else {
                            'd'
                        }
                    }
                    'q' => {
                        if d == 1 {
                            'd'
                        } else {
                            'p'
                        }
                    }
                    'p' => {
                        if d == 1 {
                            'b'
                        } else {
                            'q'
                        }
                    }
                    _ => unreachable!(),
                }
            )
            .unwrap();
        }

        writeln!(out).unwrap();
    }
}
