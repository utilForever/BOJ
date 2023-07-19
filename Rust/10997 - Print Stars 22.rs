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

fn process_stars(map: &mut Vec<Vec<char>>, depth: i64, mut row: usize, mut col: usize) {
    if depth == 1 {
        map[row][col] = '*';
        map[row + 1][col] = '*';
        map[row + 2][col] = '*';
        return;
    }

    let width = 4 * depth - 3;
    let height = 4 * depth - 1;

    for _ in 1..width {
        map[row][col] = '*';
        col -= 1;
    }

    for _ in 1..height {
        map[row][col] = '*';
        row += 1;
    }

    for _ in 1..width {
        map[row][col] = '*';
        col += 1;
    }

    for _ in 1..height - 2 {
        map[row][col] = '*';
        row -= 1;
    }

    map[row][col] = '*';
    map[row][col - 1] = '*';

    process_stars(map, depth - 1, row, col - 2);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();

    if n == 1 {
        writeln!(out, "*").unwrap();
        return;
    }

    let width = 4 * n - 3;
    let height = 4 * n - 1;
    let mut map = vec![vec![' '; width]; height];

    process_stars(&mut map, n as i64, 0, 4 * n - 4);

    for i in 0..height {
        if i == 1 {
            writeln!(out, "*").unwrap();
            continue;
        }

        for j in 0..width {
            write!(out, "{}", map[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
