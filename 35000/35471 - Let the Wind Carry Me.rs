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

    let (h, w) = (scan.token::<usize>(), scan.token::<usize>());
    let (rs, cs) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
    let (re, ce) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
    let mut grid = vec![vec!['.'; w]; h];

    for i in 0..h {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c;
        }
    }

    let dirs_wind = scan.token::<String>().chars().collect::<Vec<_>>();
    let (mut r_curr, mut c_curr) = (re as i64, ce as i64);
    let mut ret = vec![' '; dirs_wind.len()];

    for i in (0..dirs_wind.len()).rev() {
        let (r_next, c_next) = match dirs_wind[i] {
            'U' => (r_curr + 1, c_curr),
            'D' => (r_curr - 1, c_curr),
            'L' => (r_curr, c_curr + 1),
            'R' => (r_curr, c_curr - 1),
            _ => unreachable!(),
        };

        if r_next < 0 || r_next >= h as i64 || c_next < 0 || c_next >= w as i64 {
            writeln!(out, "NO").unwrap();
            return;
        }

        if grid[r_next as usize][c_next as usize] == '#' {
            ret[i] = 'G';
        } else {
            ret[i] = 'F';
            r_curr = r_next;
            c_curr = c_next;
        }
    }

    if r_curr == rs as i64 && c_curr == cs as i64 {
        writeln!(out, "YES").unwrap();
        writeln!(out, "{}", ret.into_iter().collect::<String>()).unwrap();
    } else {
        writeln!(out, "NO").unwrap();
    }
}
