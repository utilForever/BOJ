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

    let mut grid = vec![vec![String::new(); 10]; 10];

    for i in 0..10 {
        for j in 0..10 {
            grid[i][j] = scan.token::<String>();
        }
    }

    let mut ret = false;

    // Horizontal
    for i in 0..10 {
        let color = &grid[i][0];
        let mut is_same = true;

        for j in 1..10 {
            if grid[i][j] != *color {
                is_same = false;
                break;
            }
        }

        if is_same {
            ret = true;
            break;
        }
    }

    // Vertical
    for i in 0..10 {
        let color = &grid[0][i];
        let mut is_same = true;

        for j in 1..10 {
            if grid[j][i] != *color {
                is_same = false;
                break;
            }
        }

        if is_same {
            ret = true;
            break;
        }
    }

    writeln!(out, "{}", if ret { 1 } else { 0 }).unwrap();
}
