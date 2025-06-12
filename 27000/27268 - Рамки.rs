use io::Write;
use std::{io, str, vec};

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

    let (h, w, n) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut field = vec![vec!['.'; w]; h];
    let mut alphabet = 'a';

    for _ in 0..n {
        let (r1, c1, r2, c2) = (
            scan.token::<usize>() - 1,
            scan.token::<usize>() - 1,
            scan.token::<usize>() - 1,
            scan.token::<usize>() - 1,
        );

        // Top
        for c in c1..=c2 {
            field[r1][c] = alphabet;
        }

        // Bottom
        for c in c1..=c2 {
            field[r2][c] = alphabet;
        }

        // Left
        for r in r1..=r2 {
            field[r][c1] = alphabet;
        }

        // Right
        for r in r1..=r2 {
            field[r][c2] = alphabet;
        }

        alphabet = (alphabet as u8 + 1) as char;
    }

    for i in 0..h {
        for j in 0..w {
            write!(out, "{}", field[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
