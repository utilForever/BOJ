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

    let mut puzzle = [[' '; 4]; 4];

    for i in 0..4 {
        let line: String = scan.token();

        for (j, c) in line.chars().enumerate() {
            puzzle[i][j] = c;
        }
    }

    let mut ret = 0;

    for i in 0..4 {
        for j in 0..4 {
            if puzzle[i][j] == '.' {
                continue;
            }

            let c = puzzle[i][j];
            let (y, x) = (
                (c as u8 - 'A' as u8) as usize / 4,
                (c as u8 - 'A' as u8) as usize % 4,
            );

            ret += (i as i32 - y as i32).abs() + (j as i32 - x as i32).abs();
        }
    }

    writeln!(out, "{ret}").unwrap();
}
