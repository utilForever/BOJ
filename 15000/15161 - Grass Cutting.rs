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

    let k = scan.token::<i64>();
    let mut grid = [[1; 10]; 10];

    for _ in 0..k {
        let (a, b, c, d, e, f) = (
            scan.token::<usize>() - 1,
            scan.token::<usize>() - 1,
            scan.token::<usize>() - 1,
            scan.token::<usize>() - 1,
            scan.token::<usize>() - 1,
            scan.token::<usize>() - 1,
        );

        for i in 0..10 {
            grid[a][i] = 0;
            grid[b][i] = 0;
            grid[c][i] = 0;
            grid[i][d] = 0;
            grid[i][e] = 0;
            grid[i][f] = 0;
        }

        for i in 0..10 {
            for j in 0..10 {
                grid[i][j] += 1;
            }
        }
    }

    for i in 0..10 {
        for j in 0..10 {
            write!(out, "{} ", grid[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
