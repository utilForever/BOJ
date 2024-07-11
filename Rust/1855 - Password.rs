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

    let k = scan.token::<usize>();
    let s = scan.token::<String>().chars().collect::<Vec<_>>();
    let mut table = vec![vec![' '; k]; s.len() / k];

    let mut idx_s = 0;
    let mut idx_row = 0;
    let mut idx_col = 0;

    while idx_s < s.len() {
        table[idx_row][idx_col as usize] = s[idx_s];
        idx_s += 1;

        if idx_row % 2 == 0 {
            idx_col += 1;

            if idx_col == k as i64 {
                idx_row += 1;
                idx_col -= 1;
            }
        } else {
            idx_col -= 1;

            if idx_col == -1 {
                idx_row += 1;
                idx_col += 1;
            }
        }
    }

    for col in 0..k {
        for row in 0..table.len() {
            write!(out, "{}", table[row][col]).unwrap();
        }
    }

    writeln!(out).unwrap();
}
