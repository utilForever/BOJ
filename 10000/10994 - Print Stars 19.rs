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

fn process_stars(map: &mut Vec<Vec<char>>, depth: i64, row: usize, col: usize) {
    if depth == 1 {
        map[row][col] = '*';
        return;
    }

    let len = 4 * (depth as usize - 1) + 1;

    for i in col..col + len {
        map[row][i] = '*';
        map[row + len as usize - 1][i] = '*';
    }

    for i in row..row + len {
        map[i][col] = '*';
        map[i][col + len as usize - 1] = '*';
    }

    process_stars(map, depth - 1, row + 2, col + 2);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let len = 4 * (n - 1) + 1;
    let mut map = vec![vec![' '; len]; len];

    process_stars(&mut map, n as i64, 0, 0);

    for i in 0..4 * (n - 1) + 1 {
        for j in 0..4 * (n - 1) + 1 {
            write!(out, "{}", map[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
