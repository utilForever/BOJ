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

const DIRS_EVEN: [(i64, i64); 6] = [(-1, -1), (-1, 0), (0, -1), (0, 1), (1, -1), (1, 0)];
const DIRS_ODD: [(i64, i64); 6] = [(-1, 0), (-1, 1), (0, -1), (0, 1), (1, 0), (1, 1)];

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut map = vec![vec![' '; m]; n];

    for i in 0..n {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            map[i][j] = c;
        }
    }

    let mut ret = 0;

    for r in 0..n {
        for c in 0..m {
            if map[r][c] != '#' {
                continue;
            }

            let dirs = if r % 2 == 0 { DIRS_EVEN } else { DIRS_ODD };

            for &(r_curr, c_curr) in dirs.iter() {
                let r_next = r as i64 + r_curr;
                let c_next = c as i64 + c_curr;

                if r_next < 0 || r_next >= n as i64 || c_next < 0 || c_next >= m as i64 {
                    continue;
                }

                let r_next = r_next as usize;
                let c_next = c_next as usize;

                if map[r_next][c_next] == '#' {
                    continue;
                }

                ret += 1;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
