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

    // Check row
    for i in 0..n {
        let cnt_white = grid[i].iter().filter(|&&c| c == 'W').count();

        if cnt_white != n / 2 {
            writeln!(out, "0").unwrap();
            return;
        }
    }

    // Check column
    for i in 0..n {
        let cnt_white = grid.iter().filter(|&row| row[i] == 'W').count();

        if cnt_white != n / 2 {
            writeln!(out, "0").unwrap();
            return;
        }
    }

    // Check consecutive - row
    for i in 0..n {
        let mut check = false;

        grid[i].windows(3).for_each(|window| {
            if window[0] == window[1] && window[1] == window[2] {
                writeln!(out, "0").unwrap();
                check = true;
            }
        });

        if check {
            return;
        }
    }

    // Check consecutive - column
    for i in 0..n {
        let mut check = false;

        grid.windows(3).for_each(|window| {
            if window[0][i] == window[1][i] && window[1][i] == window[2][i] {
                writeln!(out, "0").unwrap();
                check = true;
            }
        });

        if check {
            return;
        }
    }

    writeln!(out, "1").unwrap();
}
