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

    let n = scan.token::<usize>();
    let mut grid = vec![vec![' '; n]; n];

    for i in 0..n {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            grid[i][j] = c;
        }
    }

    let mut ret_horizontal = 0;
    let mut ret_vertical = 0;

    // Horizontal
    for i in 0..n {
        let mut is_occupied = false;

        for j in 0..n - 1 {
            if grid[i][j] == '.' && grid[i][j + 1] == '.' && !is_occupied {
                ret_horizontal += 1;
                is_occupied = true;
            } else if grid[i][j] == 'X' {
                is_occupied = false;
            }
        }
    }

    // Vertical
    for i in 0..n {
        let mut is_occupied = false;

        for j in 0..n - 1 {
            if grid[j][i] == '.' && grid[j + 1][i] == '.' && !is_occupied {
                ret_vertical += 1;
                is_occupied = true;
            } else if grid[j][i] == 'X' {
                is_occupied = false;
            }
        }
    }

    writeln!(out, "{ret_horizontal} {ret_vertical}").unwrap();
}
