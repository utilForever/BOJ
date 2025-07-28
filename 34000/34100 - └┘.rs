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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
        let mut grid = vec![vec![' '; m]; n];

        for i in 0..n {
            let line = scan.token::<String>();

            for (j, c) in line.chars().enumerate() {
                grid[i][j] = c;
            }
        }

        let mut check = true;

        'outer: for j in 0..m {
            for i in (0..n).rev() {
                if grid[i][j] != '.' {
                    continue;
                }

                let can_fill_a =
                    i > 0 && j + 1 < m && grid[i - 1][j] == '.' && grid[i][j + 1] == '.';

                if can_fill_a {
                    grid[i][j] = 'a';
                    grid[i - 1][j] = 'a';
                    grid[i][j + 1] = 'a';
                    continue;
                }

                let can_fill_b =
                    i > 0 && j + 1 < m && grid[i][j + 1] == '.' && grid[i - 1][j + 1] == '.';

                if can_fill_b {
                    grid[i][j] = 'b';
                    grid[i - 1][j + 1] = 'b';
                    grid[i][j + 1] = 'b';
                    continue;
                }

                check = false;
                break 'outer;
            }
        }

        if !check {
            writeln!(out, "-1").unwrap();
            continue;
        }

        for i in 0..n {
            for j in 0..m {
                write!(out, "{}", grid[i][j]).unwrap();
            }

            writeln!(out).unwrap();
        }
    }
}
