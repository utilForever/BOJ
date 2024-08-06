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

fn process_dfs(pixels: &mut Vec<Vec<char>>, i: i64, j: i64) -> bool {
    if i < 0 || i >= pixels.len() as i64 || j < 0 || j >= pixels[0].len() as i64 {
        return false;
    }

    if pixels[i as usize][j as usize] == '#' {
        pixels[i as usize][j as usize] = '.';

        process_dfs(pixels, i - 1, j - 1);
        process_dfs(pixels, i - 1, j);
        process_dfs(pixels, i - 1, j + 1);
        process_dfs(pixels, i, j - 1);
        process_dfs(pixels, i, j + 1);
        process_dfs(pixels, i + 1, j - 1);
        process_dfs(pixels, i + 1, j);
        process_dfs(pixels, i + 1, j + 1);

        return true;
    }

    return false;
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (m, n) = (scan.token::<usize>(), scan.token::<usize>());
    let mut pixels = vec![vec![' '; n + 1]; m + 1];

    for i in 0..m {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            pixels[i][j] = c;
        }
    }

    let mut ret = 0;

    for i in 0..m {
        for j in 0..n {
            if process_dfs(&mut pixels, i as i64, j as i64) {
                ret += 1;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
