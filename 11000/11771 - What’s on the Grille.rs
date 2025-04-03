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

fn rotate(grille: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = grille.len();
    let mut ret = vec![vec![' '; n]; n];

    for i in 0..n {
        for j in 0..n {
            ret[i][j] = grille[n - 1 - j][i];
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut grille = vec![vec![' '; n]; n];

    for i in 0..n {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            grille[i][j] = c;
        }
    }

    let encrypted = scan.token::<String>();
    let encrypted = encrypted.chars().collect::<Vec<_>>();

    let mut grid = vec![vec![None; n]; n];
    let mut idx = 0;

    for _ in 0..4 {
        for i in 0..n {
            for j in 0..n {
                if grille[i][j] == '.' {
                    if grid[i][j].is_some() {
                        writeln!(out, "invalid grille").unwrap();
                        return;
                    }

                    grid[i][j] = Some(encrypted[idx]);
                    idx += 1;
                }
            }
        }

        grille = rotate(&grille);
    }

    if grid
        .iter()
        .any(|row| row.iter().any(|&cell| cell.is_none()))
    {
        writeln!(out, "invalid grille").unwrap();
        return;
    }

    let mut ret = String::with_capacity(n * n);

    for i in 0..n {
        for j in 0..n {
            ret.push(grid[i][j].unwrap());
        }
    }

    writeln!(out, "{ret}").unwrap();
}
