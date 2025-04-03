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

fn char_to_val(c: char) -> Option<usize> {
    if c.is_digit(10) {
        c.to_digit(10).map(|d| d as usize)
    } else if c.is_ascii_uppercase() {
        Some((c as u8 - b'A' + 10) as usize)
    } else {
        None
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut grid = vec![vec![' '; n]; n];

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c;
        }
    }

    let allowed = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let allowed = allowed.chars().take(n).collect::<Vec<_>>();

    for i in 0..n {
        let mut visited = vec![false; n];

        for j in 0..n {
            if let Some(val) = char_to_val(grid[i][j]) {
                if val >= n || visited[val] {
                    writeln!(out, "No").unwrap();
                    return;
                }

                visited[val] = true;
            } else {
                writeln!(out, "No").unwrap();
                return;
            }
        }
    }

    for j in 0..n {
        let mut visited = vec![false; n];

        for i in 0..n {
            if let Some(val) = char_to_val(grid[i][j]) {
                if val >= n || visited[val] {
                    writeln!(out, "No").unwrap();
                    return;
                }

                visited[val] = true;
            } else {
                writeln!(out, "No").unwrap();
                return;
            }
        }
    }

    let row_reduced = grid[0].iter().copied().collect::<Vec<char>>() == allowed;
    let col_reduced = (0..n).all(|i| grid[i][0] == allowed[i]);

    writeln!(
        out,
        "{}",
        if row_reduced && col_reduced {
            "Reduced"
        } else {
            "Not Reduced"
        }
    )
    .unwrap();
}
