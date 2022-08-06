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

    let (r, c) = (scan.token::<usize>(), scan.token::<usize>());
    let mut happiness = vec![vec![0; c]; r];
    let mut happiness_min = i32::MAX;
    let mut happiness_min_idx = (0, 0);

    for i in 0..r {
        for j in 0..c {
            happiness[i][j] = scan.token::<i32>();

            if (i + j) % 2 == 1 && happiness_min > happiness[i][j] {
                happiness_min = happiness[i][j];
                happiness_min_idx = (i, j);
            }
        }
    }

    if r % 2 == 1 {
        for i in 0..r {
            for _ in 0..c - 1 {
                write!(out, "{}", if i % 2 == 0 { "R" } else { "L" }).unwrap();
            }

            if i != r - 1 {
                write!(out, "D").unwrap();
            }
        }
    } else if c % 2 == 1 {
        for i in 0..c {
            for _ in 0..r - 1 {
                write!(out, "{}", if i % 2 == 0 { "D" } else { "U" }).unwrap();
            }

            if i != c - 1 {
                write!(out, "R").unwrap();
            }
        }
    } else {
        for i in 0..(happiness_min_idx.0 / 2) * 2 {
            for _ in 0..c - 1 {
                write!(out, "{}", if i % 2 == 0 { "R" } else { "L" }).unwrap();
            }

            write!(out, "D").unwrap();
        }

        let mut dr = (happiness_min_idx.0 / 2) * 2;
        let mut dc = 0;

        loop {
            if dr == (happiness_min_idx.0 / 2) * 2 + 1 && dc == c - 1 {
                break;
            }

            if dr == (happiness_min_idx.0 / 2) * 2
                && (dr + 1 != happiness_min_idx.0 || dc != happiness_min_idx.1)
            {
                write!(out, "D").unwrap();
                dr += 1;
            }

            if dc < c - 1 && (dr != happiness_min_idx.0 || dc != happiness_min_idx.1) {
                write!(out, "R").unwrap();
                dc += 1;
            }

            if dr == (happiness_min_idx.0 / 2) * 2 + 1 && dc == c - 1 {
                break;
            }

            if dr == (happiness_min_idx.0 / 2) * 2 + 1
                && (dr - 1 != happiness_min_idx.0 || dc != happiness_min_idx.1)
            {
                write!(out, "U").unwrap();
                dr -= 1;

                if dc < c - 1 && (dr != happiness_min_idx.0 || dc != happiness_min_idx.1) {
                    write!(out, "R").unwrap();
                    dc += 1;
                }
            }
        }

        for i in (happiness_min_idx.0 / 2 + 1) * 2..r {
            write!(out, "D").unwrap();

            for _ in 0..c - 1 {
                write!(out, "{}", if i % 2 == 0 { "L" } else { "R" }).unwrap();
            }
        }
    }

    writeln!(out).unwrap();
}
